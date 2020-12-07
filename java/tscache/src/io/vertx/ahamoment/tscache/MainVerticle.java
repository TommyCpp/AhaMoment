package io.vertx.ahamoment.tscache;

import io.vertx.core.Vertx;

/**
 * Created by zhongyangwu on 12/6/20.
 */
public class MainVerticle {
    public static void main(String[] args) {
        Vertx vertx = Vertx.vertx();
        vertx.deployVerticle(new BackendVerticle());
    }
}
