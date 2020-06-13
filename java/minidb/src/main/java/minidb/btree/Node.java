package minidb.btree;

/**
 * Btree Node
 */
public class Node<K extends Comparable, V> {
    private Comparable[] keys;
    private Object[] values;
    private Node<K,V>[] children;
    private int len;

    @SuppressWarnings("unchecked")
    public Node(int capacity) {
        this.keys = new Comparable[capacity];
        this.children = (Node<K,V>[])new Node[capacity + 1];
    }

    public V set(K key, V value) {
        return null;
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

    void split() {

    }
}
