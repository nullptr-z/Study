非数 DBA，记不住是正常的，会查就行

## 部门工资前三高的所有员工

关键字： dense_rank, partition, RANK, ROW_NUMBER

```sh
SELECT
  e.name AS EmployeeName,
  e.salary,
  d.name AS DepartmentName
FROM (
  SELECT
    *,
    DENSE_RANK() OVER (PARTITION BY departmentId ORDER BY salary DESC) AS ran
  FROM Employee
) e
JOIN Department d ON e.departmentId = d.id
WHERE e.ran <= 3;
```

- PARTITION BY 是 SQL 中的一个子句，通常与窗口函数（如 ROW_NUMBER()、RANK()、DENSE_RANK()、SUM() 等）一起使用

## 1251. 平均售价

[平均售价](https://leetcode.cn/problems/average-selling-price/description/?envType=study-plan-v2&envId=sql-free-50)

```sh
select product_id, ifnull(round(sum(sales) / sum(units), 2), 0) as average_price
from (
select p.product_id, price \* units as sales,u.units from Prices p
left join UnitsSold u on p.product_id = u.product_id
and u.purchase_date between p.start_date and p.end_date
) U
group by product_id
```

- ifnull: 替换 null 值，这里替换成 0
- round 保留小数位数
- 使用 group 分组时，如果没有多其他合并项处理，只会保留第一行的数据，这里进行了 sum 聚合

## 查询多少天内的时间区间

```sh
SELECT
    activity_date AS day,
    COUNT(distinct user_id) AS active_users
FROM
    Activity
WHERE
    activity_date BETWEEN DATE_ADD('2019-07-27',INTERVAL -29 day) and '2019-07-27'
GROUP BY
    activity_date
```

## DISTINCT

用于返回唯一不同的值，即从查询结果中去除重复的行。
