127.0.0.1:6379> type zz
string
127.0.0.1:6379> type xx
string
127.0.0.1:6379> incr xx
(integer) 125
127.0.0.1:6379> get xx
"125"
127.0.0.1:6379> object encoding xx
"int"
127.0.0.1:6379> object encoding zz
"embstr"
