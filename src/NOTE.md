>[!IMPORTANT]
> 此为学习笔记

# 以太网研究

## 以太网帧

我在研究以太网帧过后发现，每个标准的以太网帧都包含一个 Header 以及一个长度可以为 0 的 Payload，

Header 中又包含：

源 Mac 地址、目标 Mac 地址（也是判断是否为广播的依据）以及帧的类型（IPV4、IPV6、ARP 等）

Payload 中包含：

各个协议(IPV4、IPV6 等)的包

以下为图解
```
[ Ethernet Frame ]
    ├── Ethernet Header
    └── Payload
            ├── IPv4 Packet
            |       ├── IPv4 Header
            |       └── Payload
            ├── IPv6 Packet
            |       ├── IPv6 Header
            |       └── Payload
            └── ARP Packet
                    ├── ARP Header
                    └── ARP Data
```

## IPV4 包

IPV4 包中含一个 Header 和一个 Payload,
Header 是整个包的前 20 个字节，其中包含了：

Version(版本)、Type of Service(服务类型)、IHL(头长度)、Total Length(总长度)、

Identification(分片 ID)、Flags(分片控制位)、Fragment Offset(当前分片在原始数据中的位置)、

TTL(存活时间)、Protocol(协议类型)、Header Checksum(校验和)、

Source IP Address(源 IP 地址)、

Destination IP Address(目标 IP 地址)、

Options(选项，此为可选)、

Payload(载荷)


以下为图解 [AIGC]
```
0                   1                   2                   3
0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1

+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|Version|  IHL  |Type of Service|        Total Length         |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|       Identification          |Flags|    Fragment Offset    |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|   TTL   |    Protocol         |       Header Checksum       |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                       Source IP Address                     |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                    Destination IP Address                   |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                    Options (optional)                       |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                           Payload                           |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

在本项目里，我们只关注 Version、IHL、Total Length、TTL、Protocol、Source IP、Destination IP、Payload 这几个字段

---
本项目中发挥作用的文件
```
main.rs -> 项目入口文件

utils\bytes.rs -> 字节解析工具
ethernet.rs -> 以太网帧解析
ipv4.rs -> IPV4 协议包解析
(以下为 WIP)
ipv6.rs -> IPV6 协议包解析
arp.rs -> ARP协议包解析

```
