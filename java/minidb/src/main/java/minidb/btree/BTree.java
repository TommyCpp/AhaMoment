package minidb.btree;

import java.util.Arrays;

/**
 * Created by zhongyangwu on 6/13/20.
 */
public class BTree<K extends Comparable<K>, V> {
    private static final int C = 4; // capacity of each node, cannot be odd, must more than or equals to 4
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
            return null;
        }

        public Node set(Entry entry) {
            return this.insert(entry);
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
            } else{
                for (i = 0; i < this.num; i++) {
                    if (BTree.eq(entry.key, this.entries[i].key)) {
                        this.entries[i].value = entry.value; // update value
                        return null;
                    } else if (BTree.less(entry.key, this.entries[i].key) || (i == this.num - 1)) {
                        if(!(BTree.less(entry.key, this.entries[i].key))){
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
                this.entries[j] = this.entries[j];
            }
            this.entries[i] = entry;
            this.num++;


            return this.num == BTree.C ? this.split() : null;
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
            if(!this.isLeaf()){
                newNode.children[this.num / 2] = this.children[this.num];
            }
            this.num /= 2;
            return newNode;
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

    protected static boolean less(Comparable k1, Comparable k2) {
        return k1.compareTo(k2) < 0;
    }

    protected static boolean eq(Comparable k1, Comparable k2) {
        return k1.compareTo(k2) == 0;
    }


}
