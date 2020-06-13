package minidb.btree;

import java.util.Arrays;

/**
 * Btree LeafNode
 */
@SuppressWarnings("unchecked")
public class LeafNode<K extends Comparable, V> {
    private Comparable[] keys;
    private Object[] values;
    private int len;


    public LeafNode(int capacity) {
        this.keys = new Comparable[capacity];
    }

    /**
     * User must make sure that the LeafNode is still have enough space to insert.
     */
    public V set(K key, V value) throws FilledUpException {
        int idx = Arrays.binarySearch(this.values, key);
        if (idx >= 0) {
            V old_value = (V) this.values[idx];
            this.values[idx] = value;
            return old_value;
        } else {
            if (this.len == this.keys.length) {
                // if filled up, abort and throw filled up exception
                throw new FilledUpException();
            }
            for (int i = len; i >= -idx; i--) {
                keys[i] = keys[i - 1];
                values[i] = values[i - 1];
            }
            keys[-idx-1] = key;
            values[-idx-1] = value;
            len ++;
            return null;
        }
    }

    @SuppressWarnings("unchecked")
    public V get(K key) {
        int idx = Arrays.binarySearch(this.values, key);
        if (idx < 0) {
            return (V) this.values[idx];
        } else {
            return null;
        }
    }

    void split() {

    }
}
