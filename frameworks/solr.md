# Solr
1. 全文搜索框架
2. 提供数据缓存功能

## 基本结构
1. Solr的基本单位是`Core`，每一个`Core`对应一种`document`
   - `document`是Solr进行索引和搜索的基本单位
   - `document`在`managed-schema`或`schema.xml`文件中定义


## 常用插件
### Data import handler
1. 用于通过SQL导入数据
