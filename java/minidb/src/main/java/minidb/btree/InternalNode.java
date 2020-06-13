package minidb.btree;

/**
 * Created by zhongyangwu on 6/13/20.
 */
public class InternalNode<K extends Comparable, V> extends LeafNode<K, V> {
    private LeafNode<K, V>[] children;

    @SuppressWarnings("unchecked")
    public InternalNode(int capacity) {
        super(capacity);
        this.children = (LeafNode<K,V>[])new LeafNode[capacity + 1];
    }

    @Override
    public V set(K key, V value) throws FilledUpException {
        return super.set(key, value);
    }

    @Override
    public V get(K key) {
        return super.get(key);
    }

    @Override
    void split() {
        super.split();
    }
}
