https://discuss.leetcode.com/topic/50315/a-summary-how-to-use-bit-manipulation-to-solve-problems-easily-and-efficiently

# 维基上的定义 
比特操作是算法中用来操作变量bit的行为。计算机编程中需要比特操作的地方包括
* 低层次的设备控制
* 错误探测
* 数据压缩
* 加密算法
* 优化
* 底层协议(HTTP)

## 典型的比特操作
* AND
* OR
* XOR
* NOT（如果两个输入不同为1，否则为0）
* 比特位移

在某些情况下比特操作可以减少循环次数并且提升速度。但是因此包含比特操作的算法也更难理解和维护。

# 细节
## 基础

操作名称 |  操作符
------- | ------
Set union  | A \| B
Set intersection |  A & B
Set subtraction |  A & ~B
Set negation ALL_BITS |  ^ A or ~A
Set bit  | A |= 1 << bit
Clear bit |  A &= ~(1 << bit)
Test bit |  (A & 1 << bit) != 0
Extract last bit |  A&-A or A&~(A-1) or x^(x&(x-1))
Remove last bit |  A&(A-1)
Get all 1-bits |  ~0

## ^ 技巧（异或技巧）
* 取出同样的bit，保留不同的bit  
* 取出不同的bit，保留同样的bit

## | 技巧
* 保留近可能多的1
* 找到小于该数的，最大的2的幂

## & 技巧
* 选择特定bit

## 其他实用技巧和Tips
### 循环取出`32 bit unsigned int`中的每一位(Java)
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
### 右移和无符号右移的区别
针对负数，无符号右移在二进制最左侧补0，右移补1



