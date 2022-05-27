# collection-api

收藏名言警句/表情包/壁纸/歌词

提供上传/删除/列出/获取/随机抽取接口

> 主要是用来练习rust的模块和web框架的使用

## use-framework

- actix web后端
- seaorm 支持async/await的orm框架
- serde 序列化/反序列化
- dotenv 读取环境变量

## api doc

> 注: `scope`对应 `sentences`/`images`/`wallpaper`/`lyrics`

### json返回通用模板

+ `int` status 状态码
+ `string` message 信息
+ `object` data 下方接口对应返回内容

### 上传 `POST` `/{scope}/upload`

参数:

+ `Body application/form-url-encode`
    - `string` content 内容(图片对应url，句子对应文字内容)
    - `string?` author 作者 (图片即上传者/歌词句子即作者)
    - `string?` work 对应著作 (只在句子/歌词中有效)
    - `string` auth_key 用户验证key

返回: `application/json`

+ `int` id 数据id

### 删除 `DELETE` `/{scope}/delete`

参数:

+ `Body application/form-url-encode`
    - `int` id 数据id
    - `string` auth_key 用户验证key

返回: `application/json`

+ `int` id 数据id

### 列出 `GET` `/{scope}/list`

参数:

+ `Body application/form-url-encode`
    - `int?` offset 数据偏移量
    - `int?` limit 返回数量 默认20

返回: `application/json`

+ array
    - `int` id
    - `string` scope
    - `string` content
    - `string?` author
    - `string?` work

### 获取 `GET` `/{scope}/get`

参数:

+ `Body application/form-url-encode`
    - `int` id 对应id

返回: `application/json`

- `int` id
- `string` scope
- `string` content
- `string?` author
- `string?` work

### 随机抽取 `GET` `/{scope}/random`

返回: `application/json`

- `int` id
- `string` scope
- `string` content
- `string?` author
- `string?` work