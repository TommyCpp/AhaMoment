package minidb.btree;

/**
 * Created by zhongyangwu on 6/13/20.
 */
public class FilledUpException extends Exception {
    LeafNode a;
    LeafNode b;
    Comparable separateKey;
    Object separateVal;
}
