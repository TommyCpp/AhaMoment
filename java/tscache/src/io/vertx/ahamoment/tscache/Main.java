package io.vertx.ahamoment.tscache;

import io.vertx.core.Vertx;

/**
 * Created by zhongyangwu on 12/6/20.
 */
public class Main {
    public static void main(String[] args) {
        Vertx vertx = Vertx.vertx();
        vertx.deployVerticle(new FrontEndVerticle());
        vertx.deployVerticle(new BackendVerticle());

    }
}
