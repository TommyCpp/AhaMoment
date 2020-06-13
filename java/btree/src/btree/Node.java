package btree;

/**
 * Btree Node
 */
public class Node<K extends Comparable, V> {
    private Comparable[] keys;
    private Object[] values;
    private Node<K, V>[] children;
    private int len;

    public Node Node(int capacity) {
        Node node = new Node();
        node.keys = new Comparable[capacity];
        node.children = new Node[capacity + 1];
        return node;
    }

    public void set(K key, V value) {
    }

    @SuppressWarnings("unchecked")
    public V get(K key) {
        if (this.len == 0) {
            // empty node, return null
            return null;
        }

        // out of boundary
        if (this.keys[0].compareTo(key) < 0) {
            return this.children[0].get(key);
        }
        if (this.keys[this.len - 1].compareTo(key) > 0) {
            return this.children[this.len - 1].get(key);
        }


        for (int i = 0; i < this.len - 1; i++) {
            if (this.keys[i].compareTo(key) == 0) {
                return (V) this.values[i];
            }
            if (this.keys[i].compareTo(key) < 0 && this.keys[i + 1].compareTo(key) > 0) {
                return this.children[i + 1].get(key);
            }
        }

        return null;
    }
}
