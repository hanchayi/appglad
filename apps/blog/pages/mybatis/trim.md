---
title: The usage of mybatis's trim
---

``` typescript
interface {
  prefix: string; // add something before sql
  suffix: string; // add something after sql
  prefixOverrides: string; // remove something before sql
  suffixOverrides: string; // remove something after sql
}
```

## Remove condition `AND` or `OR`

When make some sql condition, we may use `if` tag to test some variable.

But sometimes `if` may not success, this may cause sql error.

``` xml
<if test="expression">
  xml = 1
</if>
<if test="expression1">
    AND a = 1
</if>
<if test="">
    AND b = 1
</if>
```

If expression is not matched, then the result may become

``` sql
AND a = 1 AND b = 1
```
Then we can use `trim` to resolve this problem.

``` xml
<trim prefixOverrides="AND">
  <if test="expression">
    AND xml = 1
  </if>
  <if test="expression1">
     AND a = 1
  </if>
  <if test="">
     AND b = 1
  </if>
</trim>
```

## Remove parentheses

``` xml
INSERT INTO role
  <trim suffixOverrides="," prefix="(" suffix=")">
     <if test="exp1">
        name,
     </if>
     <if test="exp2">
        age,
     </if>
  </trim>
```
