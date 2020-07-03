package minidb.btree;

/**
 * Created by zhongyangwu on 6/13/20.
 */
public class BTree<K extends Comparable<K>, V> {
    private static final int C = 4; // capacity of each node, cannot be odd, must more than or equals to 4
    private static final int M = 1; // minimum node
    private Node root;
    private int num;

    private static final class Node {
        private int num;
        private Entry[] entries;
        private Node[] children;

        public Node() {
            this.entries = new Entry[BTree.C];
            this.children = new Node[BTree.C + 1];
        }

        public Node(int num) {
            this();
            this.num = num;
        }

        public boolean isLeaf() {
            return this.children[0] == null;
        }

        public Entry get(Comparable key) {
            for (int i = 0; i < num; i++) {
                if (BTree.eq(this.entries[i].key, key)) {
                    return this.entries[i];
                } else {
                    if (BTree.less(key, this.entries[i].key)) {
                        if (this.isLeaf()) {
                            // if leaf, return null, not found
                            return null;
                        } else {
                            return this.children[i].get(key);
                        }
                    }
                }
            }
            return this.children[this.num].get(key);
        }

        private Node insert(Entry entry) {
            int i;

            if (this.isLeaf()) {
                for (i = 0; i < this.num; i++) {
                    if (BTree.eq(entry.key, this.entries[i].key)) {
                        this.entries[i].value = entry.value; // update value
                        return null;
                    } else if (BTree.less(entry.key, this.entries[i].key)) {
                        break;
                    }
                }
            } else {
                for (i = 0; i < this.num; i++) {
                    if (BTree.eq(entry.key, this.entries[i].key)) {
                        this.entries[i].value = entry.value; // update value
                        return null;
                    } else if (BTree.less(entry.key, this.entries[i].key) || (i == this.num - 1)) {
                        if (!(BTree.less(entry.key, this.entries[i].key))) {
                            i = i + 1;
                        }
                        Node node = this.children[i].insert(entry);
                        if (node != null) {
                            // add new node
                            // The child insert should split the child node into two and return the right one(which is not attach in tree)
                            // Pick the last one from left.
                            entry = this.children[i].getLast();
                            this.children[i].entries[this.children[i].num] = null;
                            // Use last as separation point
                            for (int j = this.num; j > i; j--) {
                                this.children[j] = this.children[j - 1];
                            }
                            this.children[i + 1] = node;
                        } else {
                            return null;
                        }
                        break;
                    }
                }
            }

            //do insert
            for (int j = this.num; j > i; j--) {
                this.entries[j] = this.entries[j - 1];
            }
            this.entries[i] = entry;
            this.num++;


            return this.num == BTree.C ? this.split() : null;
        }


        // Reference: https://www.geeksforgeeks.org/delete-operation-in-b-tree/
        // Return whether do we need to merge
        private boolean delete(Comparable key) {
            int i = 0;
            if (this.isLeaf()) {
                for (i = 0; i < this.num; i++) {
                    if (eq(key, this.entries[i].key)) {
                        // case 1, we just remove the node, no need to merge
                        this.pop(i);
                        return true;
                    }
                }
                return false;
            } else {
                for (i = 0; i < this.num; i++) {
                    if (eq(key, this.entries[i].key)) {
                        // Case 2, found in internal node
                        // First locate the sibling that holds
                        Node left = this.children[i]; // Child node that in left side
                        Node right = this.children[i + 1];  // Child node that in right side
                        if (left.num >= BTree.M + 1) {
                            // Case 2a, no need to merge, just delete and pull entry from left .
                            Entry newEntry = left.entries[left.num - 1];
                            left.delete(newEntry.key); // why we don't just pop it here? Bc this may not be a leaf node
                            this.entries[i] = newEntry;
                        } else {
                            if (right.num >= BTree.M + 1) {
                                // Case 2b, if left node doesn't have that many nodes, we can borrow entry from right side.
                                Entry newEntry = right.entries[0];
                                right.delete(newEntry.key);
                                this.entries[i] = newEntry;
                            } else {
                                //Case 2c, if they both don't have enough entry, merge them, and do delete recursively
                                Node keep = this.children[i];
                                Node drop = this.children[i + 1];

                                keep.entries[keep.num] = this.entries[i];
                                if(!keep.isLeaf())
                                    keep.children[keep.num + 1] = drop.children[0];
                                keep.num++;
                                for (int j = keep.num, k = 0; k < drop.num; j++, k++) {
                                    keep.entries[j] = drop.entries[k];
                                    if(!keep.isLeaf())
                                        keep.children[j + 1] = drop.children[k + 1];
                                    keep.num++;
                                }

                                this.pop(i);
                                return keep.delete(key);
                            }
                        }
                        return true;
                    } else if (less(key, this.entries[i].key) || (i == this.num - 1)) {
                        if (!less(key, this.entries[i].key)) {
                            i = i + 1;
                        }
                        boolean found = this.children[i].delete(key);
                        // Case 3, we need to search in children

                        // rebalance
                        if (found) {
                            // only need to re balance if found and delete in subtree.
                            Node left;
                            Node right;
                            if (i == this.num) {
                                left = this.children[i - 1];
                                right = this.children[i];
                            } else {
                                left = this.children[i];
                                right = this.children[i + 1];
                            }
                            if((i != this.num && left.num < BTree.M) || (i == this.num && right.num < BTree.M)){
                                if ((i != this.num && right.num >= BTree.M + 1) || (i == this.num && left.num >= BTree.M + 1)) {
                                    // Case 3a, we need to borrow from sibling
                                    Entry separateKey = right.entries[0];
                                    Node child = null;
                                    if (!right.isLeaf()) {
                                        // get child too
                                        child = right.children[0];
                                    }
                                    right.pop(0); // Why we don't delete here, bc actually here we added one entry instead of replace an entry in target. So we can carry the child node with the entry.
                                    left.entries[left.num] = this.entries[i];
                                    if (!left.isLeaf()) {
                                        left.children[left.num + 1] = child;
                                    }
                                    left.num++;
                                    this.entries[i] = separateKey;
                                } else {
                                    // Case 3b, merge separate key along with left and right
                                    if(i == this.num){
                                        i = i - 1;
                                    }
                                    left.entries[left.num] = this.entries[i];
                                    if (!left.isLeaf()) {
                                        left.children[left.num + 1] = right.children[0];
                                    }
                                    left.num++;
                                    for (int j = left.num, k = 0; k < right.num; j++, k++) {
                                        left.entries[j] = right.entries[k];
                                        if (!left.isLeaf()) {
                                            left.children[j + 1] = right.children[k + 1];
                                        }
                                        left.num++;
                                    }

                                    // Adjust this
                                    this.pop(i);
                                }
                            }
                            return true;
                        } else {
                            return false;
                        }
                    }
                }
            }

            return false;
        }


        public Entry getLast() {
            if (this.num == 0) {
                return null;
            } else {
                this.num--;
                return this.entries[this.num];

            }
        }

        private Node split() {
            Node newNode = new Node(this.num / 2);
            // always copy the last half part into new node
            for (int i = this.num / 2; i < this.num; i++) {
                newNode.entries[i - this.num / 2] = this.entries[i];
                if (!this.isLeaf()) {
                    newNode.children[i - this.num / 2] = this.children[i];
                }
            }
            if (!this.isLeaf()) {
                newNode.children[this.num / 2] = this.children[this.num];
            }
            this.num /= 2;
            return newNode;
        }


        // -- Method below is for leaf node only --

        // pop base on index
        private Entry pop(int idx) {
            if (idx < 0 || idx > this.num - 1) {
                return null;
            } else {
                Entry result = this.entries[idx];
                for (int j = idx; j < this.num; j++) {
                    this.entries[j] = this.entries[j + 1];
                    if (!this.isLeaf()) {
                        this.children[j + 1] = this.children[j + 2];
                    }
                }
                this.num--;
                return result;
            }
        }

        private void push(int idx, Entry entry) {
            if (!this.isLeaf() || idx < 0 || idx > this.entries.length || idx < this.num) {
                // Cannot replace current value
            } else {
                this.entries[idx] = entry;
                this.num++;
            }
        }

    }

    private static final class Entry {
        private Comparable key;
        private Object value;

        public Entry(Comparable key, Object value) {
            this.key = key;
            this.value = value;
        }
    }

    public BTree() {
        this.root = new Node(0);
    }

    public V get(K key) {
        Entry entry = this.root.get(key);
        if (entry != null) {
            return (V) entry.value;
        } else {
            return null;
        }
    }

    public void set(K key, V value) {
        Entry entry = new Entry(key, value);
        Node node = this.root.insert(entry);
        if (node != null) {
            // expand root node.

            Node newRoot = new Node(1);
            newRoot.entries[0] = this.root.getLast();
            newRoot.children[0] = this.root;
            newRoot.children[1] = node;

            this.root = newRoot;
        }
    }

    public void delete(K key) {
        this.root.delete(key);
        if (this.root.num == 0){
            if(this.root.children[0] != null && this.root.children[0].num > 0){
                this.root = this.root.children[0];
            } else if(this.root.children[1] != null && this.root.children[1].num > 0){
                this.root = this.root.children[1];
            }
        }
    }

    protected static boolean less(Comparable k1, Comparable k2) {
        return k1.compareTo(k2) < 0;
    }

    protected static boolean eq(Comparable k1, Comparable k2) {
        return k1.compareTo(k2) == 0;
    }


}
