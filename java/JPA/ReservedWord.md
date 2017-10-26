# Description
Some entity has field or property that use **reserved word** as name. [Reserved word in MySQL 5.5](https://dev.mysql.com/doc/refman/5.5/en/keywords.html)
When use JPA to update the entity. It can cause synax exception in SQL.

## Environment
* JPA 2.0

# Solution
When mapping field to column in entity, use `\"...\"` to wrap the reserved word.
```java
@Column(name="\"index\"")
```
## Futher
[How to wrap all field?](https://stackoverflow.com/questions/3364835/automatic-reserved-word-escaping-for-hibernate-tables-and-columns)


# Reference 
https://stackoverflow.com/questions/2224503/creating-field-with-reserved-word-name-with-jpa
