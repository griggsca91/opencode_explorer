# opencode_explorer

## Why creep on DB

Becaues i don't want to do a proxy due to the annoying way to get that work especially in an environment where you may not have easy access to certs or managing the proxy value. I'm not looking for pure accuracy, just a general idea

Yes I know this will be fragile, oh well

## Number of requests sent (excluding the initial title request that opencode sends)

the role `assistent` is the response, the `role` user is the person

```sql
select session_id, count(1) from message where data->>'role' == 'user' group by session_id;
```
