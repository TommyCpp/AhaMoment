package minidb.btree;

import java.util.Arrays;

/**
 * Created by zhongyangwu on 6/13/20.
 */
@SuppressWarnings("unchecked")
public class InternalNode<K extends Comparable, V> extends LeafNode<K, V> {
    private LeafNode<K, V>[] children;

    public InternalNode(int capacity) {
        super(capacity);
        this.children = (LeafNode<K, V>[]) new LeafNode[capacity + 1];
    }

    @Override
    public V set(K key, V value) throws FilledUpException {
        int idx = Arrays.binarySearch(this.keys, 0, this.len, key);
        if (idx >= 0) {
            // if found, update
            V old_value = (V) this.values[idx];
            this.values[idx] = value;
            return old_value;
        } else {
            try{
                this.children[-idx].set(key, value);
            } catch (FilledUpException e){
                // if filled up, split the node
            }
        }
        return null;
    }

    @Override
    public V get(K key) {
        return super.get(key);
    }

    @Override
    FilledUpException split(K key, V val) {
        return super.split(key, val);
    }

    @Override
    public boolean isLeaf() {
        return false;
    }
}
