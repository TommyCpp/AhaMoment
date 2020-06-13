package minidb.btree;

/**
 * Created by zhongyangwu on 6/13/20.
 */
public class BTree<K extends Comparable, V> {
    private Node<K,V> root;
    private int capacity;

    public BTree(int capacity){
        this.root = new Node<K,V>(capacity);
        this.capacity = capacity;
    }
}
