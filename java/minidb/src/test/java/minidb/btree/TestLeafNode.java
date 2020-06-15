package minidb.btree;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

/**
 * Created by zhongyangwu on 6/13/20.
 */
public class TestLeafNode {

    @Test
    void testTriggerSplit() {
        Assertions.assertThrows(FilledUpException.class, () -> {
            LeafNode<Integer, String> node = new LeafNode(5);
            for (int i = 0; i < 6; i++) {
                try {
                    node.set(i, "test");
                } catch (FilledUpException e) {
                    if (i != 5) {
                        Assertions.fail();
                    } else {
                        throw e;
                    }
                }
            }
        });
    }

    @Test
    void split() {
        LeafNode<Integer, String> node = new LeafNode<>(3);
        node.keys = new Integer[]{1, 2, 4};
        node.values = new String[]{"test1", "test2", "test4"};
        node.len = 3;
        try {
            node.set(3, "test3");
        } catch (FilledUpException e) {
            Assertions.assertEquals(2, e.separateKey);
            Assertions.assertEquals("test2", e.separateVal);
            Assertions.assertArrayEquals(new Integer[]{1, null, null}, e.a.keys);
            Assertions.assertArrayEquals(new Integer[]{3, 4, null}, e.b.keys);
        }


    }

}
