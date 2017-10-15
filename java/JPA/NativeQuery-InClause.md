# Description
When use JPA `Entity Manager` to create native queries with in-clause in. How to bind the parameter into the in-clause is confusing.

## Environment
* JPA2
* Hibernate

# Example
```java
String template = "SELECT * FROM journal_copy INNER JOIN copy ON copy.id = journal_copy.copy_id WHERE `status` NOT IN (?1,?2,?3)";

Query query = em.createNativeQuery(template);
 
query.setParameter(1,object);
query.setParameter(2,object);
query.setParameter(3,object);
```

# Solution
In native query, there is no way to insert in-clause dynamicly. 

That's to say, if you have 2 element in in-clause. Then you should use
```java
`status` NOT IN (?1,?2)
```
if you have 3 element in in-clause, you should use
```java
`status` NOT IN (?1,?2,?3)
```

# Reference
https://stackoverflow.com/questions/6277807/jpa-passing-list-to-in-clause-in-named-native-query

# Other
It seems that the JPQL is also doesn't support dynamic insert in-clause.
