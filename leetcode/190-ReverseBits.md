## Java中的位操作

Java中定义了如下的位操作
- 左移(<<)
- 右移(>>)
- 无符号右移(>>>)
  * 注意该操作与右移的区别，针对负数，右移在左侧补齐1，而无符号右移则补齐0
- 位与(&)
- 位或(|)
- 位非(~)
- 位异或(^)

## 实用技巧
### 循环取出`32 bit unsigned int`中的每一位
```java
int n = 1;
boolean[] result = new boolean[32]; //true for 1, false for 0
for(int i = 0;i < 32;i++){
    if(n & 1 == 1){
        result[i] = true;
    }
    else{
        result[i] = false;
    }
    n >>>= 1
}
```
