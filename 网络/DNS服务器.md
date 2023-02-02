
*属于应用协议层*
DNS服务器如何解析域名：
https://www.likecs.com/show-488715.html

　　　　　　假设 cis.poly.edu 想知道 bilibili.com 的 IP 地址。同时 cis.poly.edu 的本地 DNS 服务器为 dns.poly.edu，www.bilibili.com，如果本地 DNS 有缓存，通过查询可直接得知，它的权威域名为 ns3.dnsv5.com

完整过程：

　　　　　　1）主机  cis.poly.edu 首先向它的本地 DNS 服务器发出 DNS 查询报文，其中包含有要查询的 bilibili.com 主机名。

　　　　　　2）本地 DNS 服务器将该查询报文转发给根 DNS 服务器。

　　　　　　3）根服务器注意到 com 前缀，并返回负责 com 的 顶级域(TLD) 的 IP 地址列表。

　　　　　　4）本地 DNS 服务器再次向这些 TLD 服务器之一发送查询报文。

　　　　　　5）该 TLD 服务器注意到 dnsv5.com ，并用权威服务器的 IP 地址进行响应，该权威 DNS 服务器是 ns3.dnsv5.com。

　　　　　　6）本地 DNS 服务器之间向 ns3.dnsv5.com 重发查询报文。

　　　　　　7）ns3.dnsv5.com用 www.bilibili.com 的 IP 地址进行相应。

　　　　　　8）本地 DNS 服务器将收到的回复发回给 主机。

　　　　说明：上方的查询同时用了递归查询和迭代查询，

　　　　　　从 cis.poly.edu 到 dns.poly.edu 的查询时递归查询，因为该查询是以自己的名义请求 dns.poly.edu 来获得映射。

　　　　　　后三个查询时迭代查询，因为所有的回答都是之际返回给 dns.poly.edu 的。

