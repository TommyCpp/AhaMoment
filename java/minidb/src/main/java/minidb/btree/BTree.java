package minidb.btree;

/**
 * Created by zhongyangwu on 6/13/20.
 */
public class BTree<K extends Comparable, V> {
    private LeafNode<K,V> root;
    private int capacity;

    public BTree(int capacity){
        this.root = new LeafNode<K,V>(capacity);
        this.capacity = capacity;
    }
}
