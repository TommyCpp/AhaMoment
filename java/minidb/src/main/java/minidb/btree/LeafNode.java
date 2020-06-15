package minidb.btree;

import java.util.Arrays;

/**
 * Btree LeafNode
 */
@SuppressWarnings("unchecked")
public class LeafNode<K extends Comparable, V> {
    protected Comparable[] keys;
    protected Object[] values;
    protected int len;


    public LeafNode(int capacity) {
        this.values = new Object[capacity + 1];
        this.keys = new Comparable[capacity];
    }

    /**
     * User must make sure that the LeafNode is still have enough space to insert.
     */
    public V set(K key, V value) throws FilledUpException {
        int idx = Arrays.binarySearch(this.keys, 0, this.len, key);
        if (idx >= 0) {
            V old_value = (V) this.values[idx];
            this.values[idx] = value;
            return old_value;
        } else {
            if (this.len == this.keys.length) {
                // if filled up, abort and throw filled up exception
                throw this.split(key, value);
            }
            for (int i = len; i >= -idx; i--) {
                keys[i] = keys[i - 1];
                values[i] = values[i - 1];
            }
            keys[-idx - 1] = key;
            values[-idx - 1] = value;
            len++;
            return null;
        }
    }

    public V get(K key) {
        int idx = Arrays.binarySearch(this.values, 0, this.len, key);
        if (idx < 0) {
            return (V) this.values[idx];
        } else {
            return null;
        }
    }

    FilledUpException split(K key, V val) {
        int separateIdx = this.len / 2;

        FilledUpException exception = new FilledUpException();
        exception.separateKey = this.keys[separateIdx];
        exception.separateVal = this.values[separateIdx];
        this.keys[separateIdx] = null;
        this.values[separateIdx] = null;
        this.len = separateIdx;

        LeafNode<K, V> extraNode = new LeafNode<>(this.capacity());
        for (int i = separateIdx + 1, j = 0; i < this.capacity(); i++, j++) {
            extraNode.keys[j] = this.keys[i];
            extraNode.values[i] = this.values[i];
            extraNode.len ++;

            this.values[i] = null;
            this.keys[i] = null;
        }

        try{
            if(key.compareTo(exception.separateKey) < 0){
                this.set(key, val);
            } else{
                extraNode.set(key, val);
            }
        } catch (FilledUpException ignored){}

        exception.a = this;
        exception.b = extraNode;
        return exception;
    }

    public boolean isLeaf() {
        return true;
    }

    public int capacity() {
        return this.keys.length;
    }

}
