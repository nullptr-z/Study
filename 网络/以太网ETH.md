## 网卡

MAC 地址 6 字节， 48bit：

- 前 24bit 网卡生产商标识符
- 后 24bit 网卡厂商自定义数字

```c
#define ETHADDR_LEN 6
struct eth {
  uint8  dhost[ETHADDR_LEN]; // 目标MAC
  uint8  shost[ETHADDR_LEN]; // 源MAC
  uint16 type;               // 2byte payload的类型
}
// type 2字节
// - ETHTYPE_IP 0X0800 // Internet protocol
// - ETHTYPE_ARP 0X0806 // Address resolution protocol

```

如果购买了同一批网卡，他们前 24 为一定是相同的，后 24 位置甚至可能是连续递增的

## ARP
