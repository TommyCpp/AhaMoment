package minidb.btree;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

/**
 * Created by zhongyangwu on 6/29/20.
 */
public class BTreeTest {

    @Test
    public void testSet() {
        BTree<Integer, Integer> btree = new BTree<>();

        for (int i = 0; i < 10; i++) {
            btree.set(i, i);
        }

        Assertions.assertEquals(9, btree.get(9));

    }
}
