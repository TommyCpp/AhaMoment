#### Discussion around behavior when `X-B3-Flag` is set
So in current Golang implementation, when `X-B3-Flag` is set and `X-B3-Sampled` is not set. The span context will be set to both `Deferred` and `Debug`.

However, according to the B3 spec. The `Deferred` can only exist when there is no sampling decision. But `Debug` itself is one of the sampling decisions. So it's kind of confusing.

The behaviour of other implementations like [Brave](https://github.com/openzipkin/brave) seem to be set span context to be `Debug` and `Sampled`. 
See [here](https://github.com/openzipkin/brave/blob/6c44ccd5f545013369059e2223b48caef476c889/brave/src/main/java/brave/propagation/SamplingFlags.java#L158)
