# EasySales-Server

Started this side project to solve some ongoing issues on easysales server.

Aim to solve:

1. Real time data stream
2. An isolated object store database with the ability of memory caching. I named it `moedb`
after my cat's name 
3. Isolated HTTP and TCP server
4. TODO: Load balancer
5. Those who know about my search engine `scout`, i'm going to launch `v2` of `scout` together with this project

------

My main focus in on `moedb`. My plan is to make this similar to firebase rt database. 

------

This repo will remain as public until i start adding business logics in the code. ✌️


# Progress

**March 11**

I'm not planning to use SQL as query language. We will use redis like commands as below. 

**first draft**:

```
 1. SHOW :* -- show databases
 2. GET :moss:product {} -- get all products from db "moss" table "product"
 3. GET :moss:product {"$limit":100,"$offset":0} -- get first 100 products from db "moss" table "product"
 4. GET :moss:product {"product_code":"A001"} -- get all products where product code "A001" from db "moss" table "product"
 5. GET :moss:product {"product_available_qty":{"$bt":[10,100]}} -- get all products where available quantity from 10 to 100 from db "moss" table "product"
 6. UPSERT :moss {....}  -- insert products in db "moss" table "product"
 7. DELETE :moss:product {"product_code":"A001"}  -- delete all products where product code "A001" from db "moss" table "product"
 8. CREATE :moss -- create db "moss"
 9. CREATE :moss {                         -- create object store "person" in db "moss"
                 "name":"person",
                 "primaryKey":"id",
                 "properties":{
                     "id":"string",
                     "name":"string",
                     "age":"number",
                     "dob":"date",
                     "profile-pic":"image",
                     "bio":"blob"
                 }
             }

 10. DROP :moss -- drop db "moss"
 11. DROP :moss:product -- drop table "product" in db "moss"
 12. TRUNCATE :moss:product -- cleanup data of store "product" in db "moss"

 ```