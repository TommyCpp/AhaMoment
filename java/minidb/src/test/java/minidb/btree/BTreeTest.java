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

        for (int i = 0; i < 10; i += 2) {
            btree.set(i, i);
        }

        for (int i = 1; i < 10; i += 2) {
            btree.set(i, i);
        }


        for (int i = 0; i < 10; i++) {
            Assertions.assertEquals(i, btree.get(i));
        }

        btree.set(5, 50);
        Assertions.assertEquals(50, btree.get(5));

    }
}
