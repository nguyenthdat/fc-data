<!-- PAGE 1 -->

# FastConnectData API Specs V 2.1

> Converted from `SSI_FastConnectData_Specs_v2.2.pdf`. HTML comments preserve the source page boundaries.

<!-- PAGE 2 -->

## Contents
2. **Introduction ......................................................................................................................................... 5**
3. **Environment information ................................................................................................................... 5**
4. **Market data streaming ........................................................................................................................ 5 3.1 Securities Status ......................................................................................................................... 5 3.2 Market Data .................................................................................................................................. 6 3.3 Foreign Room Data ..................................................................................................................... 9 3.4 Index Data .................................................................................................................................. 10 3.5 Realtime Bars ............................................................................................................................ 12**
5. **Market Data API ................................................................................................................................. 12 4.1 Auth ............................................................................................................................................ 12**

Auth Request....................................................................................................................................... 12 Auth Response .................................................................................................................................... 13 Example 13

**4.2 Securities ................................................................................................................................... 13**

Securities List Request ....................................................................................................................... 13 Securities List Response ..................................................................................................................... 14 Example 14

**4.3 SecuritiesDetails (GetSecuritiesDetails) ................................................................................. 14**

Securities Details Request .................................................................................................................. 14 Securities Details Response ............................................................................................................... 15 Example 16

**4.4 Index Component (GetIndexComponents) ............................................................................. 17**

Index Component Request ................................................................................................................. 18 Index Component Response ............................................................................................................... 18 Example 18

**4.5 Index List (GetIndexList) .......................................................................................................... 19**

Index List Request .............................................................................................................................. 19 Index List Response ............................................................................................................................ 19 Example 20

**4.6 Daily OHLC (GetDailyOHLC) .................................................................................................... 20**

GetDailyOHLC Request ...................................................................................................................... 20 GetDailyOHLC Response ................................................................................................................... 21 Example 21

**4.7 Intraday OHLC (GetIntradayOHLC) ......................................................................................... 22**

GetIntradayOHLC Request ................................................................................................................. 22 GetIntradayOHLC Response .............................................................................................................. 22 Example 22

**4.8 Daily Index (GetDailyIndex) ...................................................................................................... 23**

GetDailyIndex Request ....................................................................................................................... 23 GetDailyIndex Response .................................................................................................................... 24 Example 25

**4.9 Daily Stock Price (GetStockPrice) ........................................................................................... 25**

GetStockPrice Request ....................................................................................................................... 25 GetStockPrice Response .................................................................................................................... 26 Example 27

<!-- PAGE 3 -->

_This page is blank in the source PDF._

<!-- PAGE 4 -->

## Revision history

| Date | Changed by | Description | Version |
| --- | --- | --- | --- |
| 04/05/2020 |  | Created | 1.0 |
| 26/10/2020 |  | Update FC data to new version | 1.1 |
| 04/01/2021 |  | Update FC data API AccessToken | 2.0 |
| 02/04/2021 |  | Add EstIndexVal field into 3.4 | 2.0 |
| 06/04/2021 |  | Update separate msg Marketdata into Msg Trade Data and Quote data | 2.0 |
| 10/05/2022 |  | Bổ sung input Streaming MarketData | 2.1 |
| 26/07/2023 |  | Update lại link Streaming FCData | 2.2 |

<!-- PAGE 5 -->

# 1. Introduction

**FastConnect Data** is one of the feature of our Application Programming Interface (i.e API), which is an environment enabling our customers able to trade and/or perform analysis by their own applications. FC Data includes the market data and fundamental information. Depending on the user accessibility level, user may get daily closing information of the securities, intra-day securities detail We support .Net, Javascript, Java and Python environments.

# 2. Environment information

| Service Name | Env Name | Host | Port | Requires VPN |
| --- | --- | --- | --- | --- |
| DataAPI | Prod | https://fc- |  |  |
|  |  | datahub.ssi.com.vn |  |  |

# 3. Market data streaming

Data Streaming in FastConnect is taken care by its sub-system, Information Distribution Service (IDS). IDS is a subscription basis data pushing facility. This version of IDS focuses on the streaming of market data as well as status of active orders. In the near future, more information will be added such as company profile, back testing facilities, and others

## 3.1 Securities Status

**Input:** F: symbols **Example** : F:SSI or F:SSI-PAN or F:ALL

**Output STT Name of Element Type Description Value/ Format**

1 Rtype String This is a fixed value to F: Securities Status identify the value of this record for what type of information 3 TradingDate Date Date & time of data record DDMMYYYY

| 2 | MarketId |  | The ID of their | HOSE |
| --- | --- | --- | --- | --- |
|  |  |  | corresponding Exchange | HNX |
|  |  |  | where the equity is trading | HNXBOND |
|  |  |  |  | UPCOM |
|  |  |  |  | DER |

|  | Time | Time |  | 24HHMMSS |
| --- | --- | --- | --- | --- |
| 4 | Symbol | String | Ticker of the securities | Symbol of stock |

<!-- PAGE 6 -->

5 TradingSession String Session status of the Have the following value: market ATO: Opening Call Auction LO: Continuous Trading ATC: Closing All Auction PT: Putthrough C: Market Close BREAK: Lunch Break HALT: Market Halt 6 TradingStatus String Have the following values:

N: Normal – Giao dịch bình thường D: Delisted – Bị hủy niêm yết H: Halt – Tạm dừng giao dịch giữa phiên S: Suspend – Ngừng giao dịch NL: New List – Niêm yết mới ND: Sắp hủy niêm yết ST: Special Trading – Giao dịch đặc biệt SA: Suspend A – Bị ngưng giao dịch khớp lệnh SP: Suspend PT – Ngừng giao dịch khớp lệnh thỏa thuận Exchange HOSE HNX

**Example:**

{ datatype: 'F', content:

'{"Rtype":"F","MarketId":"HNX","TradingDate":"04/05/2020","Time":"15:00:16","Symbol":"DPS","TradingSes sion":"C","TradingStatus":"NT","Exchange":"HNX"}' }

## 3.2 Market Data

**Separate msg Marketdata into Msg Trade Data and Quote data Input : X: Symbol VD: X-QUOTE: ALL, X-TRADE: ALL Output:**

**No. Name of Element Type Description Valid Value or Format**

1. Rtype String X: Stock price

| 2. | TradingDate | Date |  |  |  |
| --- | --- | --- | --- | --- | --- |
| 3. | Time | Time |  |  |  |

<!-- PAGE 7 -->

1. ISIN String ISIN code
2. Symbol String Local ticker Symbol of stock
3. Ceiling Number Ceiling price
4. Floor Number Floor price
5. RefPrice Number Reference price
6. Open Number The opening price of the equity
7. Close Number The closing price of the equity
8. High Number Highest matched price in the date of report
9. Low Number Lowest matched price in the date of report.
10. Avg Number Average price of the security
11. PriorVal Number The closing price of previous trading day of the report date.
12. LastPrice Number Latest matched price
13. Change Number Change
14. RatioChange Number Ratio Change
15. EstttiatchedPrice Number Estimate Matched Price
16. LastVol Number Latest matched volume
17. TotalVal Number Total matched value from market opening till the reporting time.
18. TotalVol Number Total matched volume from market opening till the reporting time.
19. BidPrice1 Number Best bid price 1
20. BidVol1 Number Best bid volume 1
21. BidPrice2 Number Best bid price 2
22. BidVol2 Number Best bid volume 2
23. BidPrice3 Number Best bid price 3
24. BidVol3 Number Best bid volume 3
25. BidPrice4 Number Best bid price 4
26. BidVol4 Number Best bid volume 4
27. BidPrice5 Number Best bid price 5
28. BidVol5 Number Best bid volume 5
29. BidPrice6 Number Best bid price 6
30. BidVol6 Number Best bid volume 6
31. BidPrice7 Number Best bid price 7
32. BidVol7 Number Best bid volume 7
33. BidPrice8 Number Best bid price 8
34. BidVol8 Number Best bid volume 8
35. BidPrice9 Number Best bid price 9
36. BidVol9 Number Best bid volume 9
37. BidPrice10 Number Best bid price 10
38. BidVol10 Number Best bid volume 10

| 42. | AskPrice1 | Number | Best Ask price 1 |  |
| --- | --- | --- | --- | --- |
| 43. | AskVol1 | Number | Best Ask volume 1 |  |

<!-- PAGE 8 -->

1. AskPrice2 Number Best Ask price 2
2. AskVol2 Number Best Ask volume 2
3. AskPrice3 Number Best Ask price 3
4. AskVol3 Number Best Ask volume 3
5. AskPrice4 Number Best Ask price 4
6. AskVol4 Number Best Ask volume 4
7. AskPrice5 Number Best Ask price 5
8. AskVol5 Number Best Ask volume 5
9. AskPrice6 Number Best Ask price 6
10. AskVol6 Number Best Ask volume 6
11. AskPrice7 Number Best Ask price 7
12. AskVol7 Number Best Ask volume 7
13. AskPrice8 Number Best Ask price 8
14. AskVol8 Number Best Ask volume 8
15. AskPrice9 Number Best Ask price 9
16. AskVol9 Number Best Ask volume 9
17. AskPrice10 Number Best Ask price 10
18. AskVol10 Number Best Ask volume 10
19. MarketID value:

| 63. | Exchange |  |  |  |
| --- | --- | --- | --- | --- |
| 64. | TradingSession | String | Session status of the market | Having the following |

- ATO: Opening Call Auction
- LO: Continuous Trading
- ATC: Closing Call Auction
- PT: Putthrough
- C: Market Close
- BREAK: Lunch Break - H: Market Halt
- TradingStatus String Having the following value:
- N: Normal – Giao dịch bình thường
- D: Delisted - Bị hủy niêm yết
- H: Halt - Tạm dừng giao dịch giữa phiên
- S: Suspend - Ngừng giao dịch
- NL: New List - Niêm yết mới
- ND: Sắp hủy niêm yết

<!-- PAGE 9 -->

- ST: Special Trading
- Giao dịch đặc biệt
- SA: Suspend A – Bị ngưng giao dịch khớp lệnh
- SP: Suspend PT – Ngừng giao dịch khớp lệnh thỏa thuận

**Example:**

- Msg Quote data: {"DataType":"Quote","Content":"{ {"RType:"Quote",\\"TradingDate\\":\\"06/04/2021\\",\\"TradingTime\\":\\"08 :54:52\\",\\"Exchange\\":\\"DERIVATIVES\\",\\"Symbol\\":\\"VN30F2104\\",\\"StockNo\\":\\"1138\\",\\"AskPrice1\\": 0.0,\\"AskPrice2\\":1252.0,\\"AskPrice3\\":1254.0,\\"AskPrice4\\":1257.0,\\"AskPrice5\\":1257.5,\\"AskPrice6\\" :1258.6,\\"AskPrice7\\":1259.0,\\"AskPrice8\\":1259.4,\\"AskPrie9\\":1259.5,\\"AskPrice10\\":1259.8,\\"AskVol 1\\":\\"140\\",\\"AskVol2\\"\\"1\\",\\"AskVol3\\":\\"1\\",\\"AskVol4\\":\\"6\\",\\"AskVol5\\":\\"20\\",\\"AskVol6\\":\\"10\\",\\"Ask Vol7\\":\\"8\\",\\"AskVol8\\":\\"2\\",\\"AskVol9\\":\\"6\\",\\"AskVol10\\":\\"2\\",\\"BidPrice1\\":0.0,\\"BidPrice2\\":1344.9,\\" BidPrice3\\":1265.0,\\"BidPrice4\\":1261.4,\\"BidPrice5\\":1260.1,\\"BidPrice6\\":1260.0,\\"BidPrice7\\":1259.4 ,\\"BidPrice8\\":1258.5,\\"BidPrice9\\":1258.0,\\"BidPrice10\\":1257.1,\\"BidVol1\\":\\"150\\",\\"BidVol2\\":\\"18\\",\\" BidVol3\\":\\"1\\",\\"BidVol4\\":\\"1\\",\\"BidVol5\\":\\"1\\",\\"BidVol6\\":\\"33\\",\\"BidVol7\\":\\"1\\",\\"BidVol8\\":\\"1\\",\\"Bid Vol9\\":\\"10\\",\\"BidVol10\\":\\"4\\",\\"StockType\\":\\"Future\\"}"}
- Msg Trade data { datatype: 'Trade', content: {"RType:"Trade","TradingDate":"04/05/2020","Time":"14:46:51","ISin":null,"Symbol":"SSI","Ceiling":13900, "Floor":12100,"RefPrice":13000,"Open":12900,"Close":12700,"High":13000,"Low":12700,"AvgPrice":1281 6,"PriorVal":13000,"LastPrice":12700,"LastVol":2180310,"TotalVal":27943000000,"TotalVol":2180310,"Ma rketId":"HOSE","Exchange":"HOSE","BidPrice1":12650,"BidVol1":37330,"BidPrice2":12600,"BidVol2":507 70,"BidPrice3":12550,"BidVol3":2720,"BidPrice4":0,"BidVol4":0,"BidPrice5":0,"BidVol5":0,"BidPrice6":0,"Bi dVol6":0,"BidPrice7":0,"BidVol7":0,"BidPrice8":0,"BidVol8":0,"BidPrice9":0,"BidVol9":0,"BidPrice10":0,"Bid Vol10":0,"AskPrice1":12700,"AskVol1":51670,"AskPrice2":12750,"AskVol2":18260,"AskPrice3":12800,"As kVol3":47210,"AskPrice4":0,"AskVol4":0,"AskPrice5":0,"AskVol5":0,"AskPrice6":0,"AskVol6":0,"AskPrice7 ":0,"AskVol7":0,"AskPrice8":0,"AskVol8":0,"AskPrice9":0,"AskVol9":0,"AskPrice10":0,"AskVol10":0,"Tradin gSession":"ATC","TradingStatus":"N","Change":-300,"RatioChange":-2.31,"EstttiatchedPrice":0}' }

## 3.3 Foreign Room Data

**Input** : Rtisymbols **Example**: R:SSI or R:SSI-PAN or R:ALL

**No. Name of Element Type Description Valid Value or Format**

1. Rtype String Message type R: Foreign room
2. TradingDate Date
3. Time Time

| 4. | ISIN | String | ISIN code |  |
| --- | --- | --- | --- | --- |
| 5. | Symbol | String | Local ticker |  |

<!-- PAGE 10 -->

1. TotalRoom Number Available no of shares for foreign trading.
2. CurrentRoom Number Total matched volumes which foreigner brought from market opening till the reporting time.
3. FBuyVol Number Total matched volumes which foreigner buy from market opening till the reporting time. customers streaming tự tính theo công thức: BuyVal = BuyVol * LastPrice (Giá khớp gần nhất)

| 9. | FSellVol |  | Total matched volumes which |  |
| --- | --- | --- | --- | --- |
|  |  |  | foreigner sold from market opening till |  |
|  |  |  | the reporting time. |  |
| 10. | FBuyVal | Number | Total matched buy value of foreign | Đối với HOSE, |

1. FSellVal Number Total matched sell value of foreign Đối với HOSE, customers streaming tự tính theo công thức: SellVal = SellVol * LastPrice (Giá khớp gần nhất)

**Example:**

{ datatype: 'R', content: '{"RType":"R","TradingDate":"04/05/2020","Time":"15:02:45","Isin":"YTC","Symbol":"YTC","TotalRoom":0. 0,"CurrentRoom":1475400.0,"BuyVol":0.0,"SellVol":0.0,"BuyVal":0.0,"SellVal":0.0,"MarketId":"UPCOM","E xchange":"UPCOM"}' }

## 3.4 Index Data

**Input:** MI:VN30 or MI:VN30-HNXindex or MI:ALL

**Output:**

**No Name of Element Tyoe Description Valid value**

1. Rtype String Message Type MI: Index Information
2. IndexID String Index ID
3. IndexValEst Number Value of the Estimate Index
4. IndexValue Number Value of the index
5. Trading Date Date Trading Date
6. Time Timestamp Time
7. Change Number Change of index

| 8. | RatioChange | Number | Ratio of Change |  |
| --- | --- | --- | --- | --- |
| 9. | TotalTrade | Number | Tổng số lệnh khớp (cả thông thường |  |
|  |  |  | và thỏa thuận) |  |

<!-- PAGE 11 -->

1. TotalQtty Number Tổng matched quantity of normal orders such as VN30 "Industry" – industrial related index "Other" – non classified index.

| 11. | TotalValue | Number | Tổng matched value of normal |  |
| --- | --- | --- | --- | --- |
|  |  |  | orders |  |
| 12. | TypeIndex | String | Index | “Main” – main index |

1. IndexName String Name of Index
2. Advances Number Total number of symbols having price increase
3. Nochanges Number Total number of symbols having price unchange
4. Declines Number Total number of symbols having price decrease
5. Ceiling Number Total number of symbols having last price = ceiling price
6. Floor Number Total number of symbols having last price = floor price
7. TotalQttyPT Number Total matched quantity of puthrough orders
8. TotalValuePT Number Total matched value of putthrough orders
9. TotalQttyOd Number
10. TotalValueOd Number
11. AllQty Number Total matched quantity of both normal and putthrough
12. AllValue Number Total matched value of both normal and putthrough
13. TradingSession String Trading session - ATO: ATO
14. LO: Continuous
15. ATC: ATC
16. PT: Putthrough
17. BREAK: lunch break
18. C: Market Close

|  |  |  |  | - H: Market halted |
| --- | --- | --- | --- | --- |
| 26. | Market | String | Market | - HOSE |

- HNX
- Exchange String Exchange - HOSE
- HNX

**Example:**

{'DataType': 'MI',

<!-- PAGE 12 -->

'Content': {"IndexId":"VN30","IndexValEst":1200.03,"IndexValue":1238.76,"PriorIndexValue":1226.16,"TradingDate": "02/04/2021","Time":"11:28:13","TotalTrade":0.0,"TotalQtty":191838100.0,"TotalValue":7289093000000.0 ,"IndexName":"VN30","Advances":25,"NoChanges":2,"Declines":3,"Ceilings":0,"Floors":0,"Change":12.6," RatioChange":1.03,"TotalQttyPt":2064000.0,"TotalValuePt":244251000000.0,"Exchange":"HOSE","AllQty ":193902100.0,"AllValue":7533344000000.0,"IndexType":"Main","TradingSession":null,"MarketId":null,"RT ype":"MI","TotalQttyOd":0.0,"TotalValueOd":0.0}'}

## 3.5 Realtime Bars

**Input:** B:SSI; B:SSI-VN30 or B:ALL

**Output:**

**No. Name of Element Type Description Valid Value or Format**

1. Rtype String Message Type b: OHLC information Derivatives

| 2. | Time | Timestamp | Include date and time |  |
| --- | --- | --- | --- | --- |
| 3. | Symbol | String | Local ticker | Include both Index, Stock and |

1. Open Number Open price
2. High Number Highest price
3. Low Number Lowest price
4. Close Number Close price

| 8. | Volume | Number | Matched Volume | KL khớp gần nhất |
| --- | --- | --- | --- | --- |
| 9. | Value | Number | Matched Value | GT khớp gần nhất. Tạm thời |
|  |  |  |  | để bằng 0 |

**Example:**

{'Datatype': 'B', 'Content': '{"RType":"B","Symbol":"X26","TradingTime":"14:28:33","Open":16000,"High":16000,"Low":16000,"Close": 16000,"Volume":5000,"Value":0}' }

# 4. Market Data API

## 4.1 Auth

- Url: https://fc-data.ssi.com.vn/api/v2/Market/AccessToken
- Method: POST

### Auth Request

**No. Name of Type Type Description Element**

Header

1. Content-Type String Y application/json
2. Accept Number Y application/json Body
3. consumerID string Y ConsumerID in the connection information
4. consumerSecret string Y ConsumerSecret in the connection information

<!-- PAGE 13 -->

### Auth Response

**No. Name of Type Description Valid Value or Format Element**

1. status String
2. message String
3. data String accessToken

### Example

**Request Response**

{ { "consumerID": "c058f5576181478788 "responseCode": 0, 2b2c8df1336e25", "consumerSecret": "message": "Success", "144cac45770949519 d2dfd20edb5b6ab", "token": "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9" } } Token này sẽ dùng để truyền các api từ 4.2 đến 4.9 với format như sau:

**No. Name of Type Required Description Valid Value or Format Element**

Header

| 1. |  |  | String | Y | Bearer {Token} |  |
| --- | --- | --- | --- | --- | --- | --- |
| 2. |  |  | Number | Y | application/json |  |

## 4.2 Securities

- Url: https://fc-data.ssi.com.vn/api/v2/Market/Securities
- Method: GET

### Securities List Request

1. Market String N Input market to get - HOSE securities list belonging to - HNX that market - UPCOM
2. DER If not specify then returns all market
3. Pageindex Number Y From 1 to 10 Default 1

<!-- PAGE 14 -->

1. Pagesize Number 10; 20; 50; 100; 1000 Default 10

### Securities List Response

**No. Name of Type Description Valid Value or Format Element**

1. Market String Stock exchanges
2. Symbol String Ticker of the securities

| 6. |  |  | String | Stock Name in Vietnamese |  |
| --- | --- | --- | --- | --- | --- |
| 7. |  |  | String | Stock Name in English |  |

### Example

**Request Response**

{ { "data": [ pageIndex : "1" { "Market": "HOSE", "Symbol": "AAA", market: "hose" "StockName": "CTCP NHUA\&MT XANH AN PHAT", "StockEnName": "An Phat Bioplastics Joint Stock Company" } }, { "Market": "HOSE", "Symbol": "AAM", "StockName": "CTCP THUY SAN MEKONG", "StockEnName": "Mekong Fisheries Joint Stock Company" } ], "message": "Success", "status": "Success", "totalRecord": 560 }

## 4.3 SecuritiesDetails (GetSecuritiesDetails)

- Url: https://fc-data.ssi.com.vn/api/v2/Market/SecuritiesDetails
- Method: GET

### Securities Details Request

**No. Name of Element Type Required Description Valid Value or Format**

<!-- PAGE 15 -->

1. 3.

| 2. | Symbol | String | N |  | If not specify then |
| --- | --- | --- | --- | --- | --- |
|  |  |  |  |  | returns all market |

4\.

### Securities Details Response

**No.**

1\. 2. 3.

**4.**

| 5. | ISIN | String | ISIN code of the securities | Not used yet |
| --- | --- | --- | --- | --- |
| 6. | Symbol | String | The local trading code of | Equity listed in the exchanges. |

7\. 10.

Market pageIndex

lookupRequest.pageSize

**Name of Element**

RType ReportDate TotalNoSym

**Repeated info**

SymbolName MarketID

String N integer N

integer integer

**Type Description**

String This is a fixed value to identify the value of this record for equity information. Date Date & time of the data record. Number Total number of symbol returned the equity listed in the exchanges String Name of the securities String The market of the securities

- HOSE
- HNX
- UPCOM
- DER If not specify then returns all market From 1 to 10

Default 1 10; 20; 50; 100; 1000 Default 10

**Valid Value or Format**

"y" – Security List dd/MM/yyyy HHtimmtiss

| 8. | SymbolEngName | String | Name in English |  |
| --- | --- | --- | --- | --- |
| 9. | SecType | String | The type of equity. | - ST: Stock |

- CW: Covered Warrant
- FU: Futures
- EF: ETF
- BO: BOND
- OF: OEF
- MF: Mutual Fund
- HOSE
- HNX
- HNXBOND
- UPCOM
- DER

<!-- PAGE 16 -->

1. Exchange String The ID of the - HOSE: Hochiminh Stock corresponding Exchange Exchange where the equity is trading. - HNX: Hanoi Stock Exchange
2. Issuer String Issuer of the security
3. LotSize Number Trading lot size of the security
4. IssueDate Date
5. MaturityDate Date
6. FirstTradingDate Date
7. LastTradingDate Date securities P: Physical

| 18. | ContractMultiplier | Number | Contract Multiplier |  |
| --- | --- | --- | --- | --- |
| 19. | SettlMethod | String | Settlement method of the | C: Cash |

| 20. | Underlying | String | Underlying securities |  |
| --- | --- | --- | --- | --- |
| 21. | PutOrCall | String | Option Type | - P: Put |

- C: Call CW, Options - A: American Only available with CoverWarrant

| 22. | ExercisePrice | Number | Exercise price. Used for |  |
| --- | --- | --- | --- | --- |
|  |  |  | Options, CW |  |
| 23. | ExerciseStyle | String | Exercise Style. Used for | - E: European |

1. ExcerciseRatio String Exercise ratio, used for CW, Options
2. ListedShare Number Number of listed shares
3. TickPrice1 Number Starting price range 1 for tick rule
4. TickIncrement1 Number Tick increasement for price range 1 for tick rule
5. TickPrice2 Number Starting price range 2 for tick rule
6. TickIncrement2 Number Tick increasement for price range 2
7. TickPrice3 Number Starting price range 3 for tick rule
8. TickIncrement3 Number Tick increasement for price range 3

| 32. | TickPrice4 | Number | Starting price range 4 for |  |
| --- | --- | --- | --- | --- |
|  |  |  | tick rule |  |
| 33. | TickIncrement4 | Number | Tick increasement for price |  |
|  |  |  | range 4 |  |

### Example

**Request Response**

<!-- PAGE 17 -->

Symbol: SSI { "dataList": [ { "rtype": "y", "reportdate": "04/05/2020", "totalnosym": "1", "repeatedinfoList": [ { "isin": "", "symbol": "SSI", "symbolname": "Công ty Cổ phần Chứng khoán SSI", "symbolengname": "SSI Securities Corporation", "sectype": "ST", "marketid": "HOSE", "exchange": "HOSE", "issuer": "Công ty Cổ phần Chứng khoán SSI", "lotsize": "10", "issuedate": "", "maturitydate": "", "firsttradingdate": "", "lasttradingdate": "", "contractmultiplier": "0", "settlmethod": "C", "underlying": "", "putorcall": "", "exerciseprice": "0", "exercisestyle": "", "excerciseratio": "0", "listedshare": "602952421", "tickprice1": "1", "tickincrement1": "10", "tickprice2": "10000", "tickincrement2": "50", "tickprice3": "50000", "tickincrement3": "100", "tickprice4": "", "tickincrement4": "" } ] } ], "message": "SUCCESS", "status": "SUCCESS", "totalrecord": 1, "securitiesdetailsresponse": { } }

## 4.4 Index Component (GetIndexComponents)

- Url: https://fcdata.ssi.com.vn/api/v2/Market/IndexComponents
- Method: GET

<!-- PAGE 18 -->

### Index Component Request

**No Name of Type Mandatory Description Valid Value Element**

1. Indexcode String Y Input Index Code to get consituent stocks
2. PageIndex Number Y From 1 to 10 Default 1
3. PageSize Number Y 10; 20; 50; 100; 500; 1000 Default 10

### Index Component Response

**No Name of Element Type Description Valid Value**

1. IndexCode String Index Code
2. IndexName String
3. Exchange String Exchange of the Index
4. TotalSymbolNo Number Total number of symbols in the Index List of stocks with the following information

| 5. | ISIN | String | ISIN Code of the security |  |
| --- | --- | --- | --- | --- |
| 6. | StockSymbol | String | Ticker code of the security |  |

### Example

**Request Response**

<!-- PAGE 19 -->

Indexcode: VN30 { "dataList": [ { "indexcode": "VN30", "indexname": "VN30", "exchange": "HOSE", "totalsymbolno": "30", "indexcomponentList": [ { "isin": "BID", "stocksymbol": "BID" }, … { "isin": "BVH", "stocksymbol": "BVH" } ] } ], "message": "SUCCESS", "status": "SUCCESS", "totalrecord": 30, "indexcomponentresponse": { } }

## 4.5 Index List (GetIndexList)

- Url: https://fc-data.ssi.com.vn/api/v2/Market/IndexList
- Method: GET

### Index List Request

| 2. PageIndex | Number N | From 1 to 10 Default 1 |  |
| --- | --- | --- | --- |
| 3. PageSize | Number N | 10; 20; 50; | 100; 500; 1000 |
|  |  | Default 10 |  |

### Index List Response

**No Name of Element Type Description Valid Value**

Returns list of indexes with the following fields

<!-- PAGE 20 -->

1. Indexcode String Index Code

| 2. | Indexname | String | Index Name |  |
| --- | --- | --- | --- | --- |
| 3. | Exchange | String | Exchange of the Index |  |

### Example

**Request Response**

Exchange: HNX { "dataList": [ { "indexcode": "HNX30", "indexname": "HNX30", "exchange": "HNX" }, { "indexcode": "HNXIndex", "indexname": "HNXIndex", "exchange": "HNX" }, { "indexcode": "HNXUpcomIndex", "indexname": "HNXUpcomIndex", "exchange": "HNX" } ], "message": "SUCCESS", "status": "SUCCESS", "totalrecord": 3, "indexlistresponse": { } }

## 4.6 Daily OHLC (GetDailyOHLC)

- Url: https://fc-data.ssi.com.vn/api/v2/Market/DailyOhlc
- Method: GET

### GetDailyOHLC Request

**No Name of Type Mandatory Description Valid Value Element**

1. Symbol String N Symbol of stock, indexcode, derivatives
2. FromDate Date Y If not specify, get today date
3. ToDate Date Y If not specify, get today date. Max range 30 days
4. PageIndex Number N From 1 to 10

<!-- PAGE 21 -->

Default 1

1. PageSize Number N 10; 20; 50; 100; 500; 1000 Default 10
2. ascending boolean Yes true/ false

### GetDailyOHLC Response

**No Name of Element Type Description Valid value**

1. Symbol String Index ID/Stock Symbol
2. Market String Market of the symbol
3. Trading Date Date Trading Date
4. Time Timestamp Time
5. Open Number
6. High Number
7. Low Number
8. Close Number

| 9. | Volume | Number | Total normal matched |  |
| --- | --- | --- | --- | --- |
|  |  |  | volume |  |
| 10. | Value | Number | Total normal matched |  |
|  |  |  | value |  |

### Example

**Request Response**

Symbol: SSI Fromdate: 04/05/2020 "dataList": [ Todate: 04/05/2020 { "symbol": "SSI", "market": "HOSE", "tradingdate": "04/05/2020", "time": "", "open": "12900", "high": "13000", "low": "12700", "close": "12700", "volume": "2180310", "value": "27943000000" } ], "message": "SUCCESS", "status": "SUCCESS", "totalrecord": 1, "dailyohlcresponse": { } }

<!-- PAGE 22 -->

## 4.7 Intraday OHLC (GetIntradayOHLC)

- Url: https://fc-data.ssi.com.vn/api/v2/Market/IntradayOhlc
- Method: GET

### GetIntradayOHLC Request

**No Name of Type Mandatory Description Valid Value Element**

1. Symbol String Y Input Index ID or Instrument Symbol to get data. Including Stock, Derivatives, CW
2. FromDate Date N If not specify, get today date
3. ToDate Date N If not specify, get today date
4. PageIndex Number N From 1 to 10 Default 1
5. PageSize Number N 10; 20; 50; 100; 500; 1000 Default 10

| 6. |  |  | integer | N |  | Group data by 1m |
| --- | --- | --- | --- | --- | --- | --- |
| 7. |  |  | boolean | N |  |  |

### GetIntradayOHLC Response

| No |  |  | Tyoe | Description | Valid value |
| --- | --- | --- | --- | --- | --- |
| 1. |  |  | String | Index ID/Stock Symbol |  |
| 2. |  |  | Number | Value of the index |  |
| 3. |  |  | Date | Trading Date |  |
| 4. |  |  | Timestamp | Time |  |
| 5. |  |  | Number |  |  |
| 6. |  |  | Number |  |  |
| 7. |  |  | Number |  |  |
| 8. |  |  | Number |  |  |
| 9. |  |  | Number |  |  |
| 10. |  |  | Number |  |  |

### Example

**Request Response**

Symbol: SSI { Fromdate: 04/05/2020 "dataList": [ Todate: 04/05/2020 {

<!-- PAGE 23 -->

"symbol": "SSI", "value": "12900", "tradingdate": "04/05/2020", "time": "09:15:44", "open": "12900", "high": "12900", "low": "12900", "close": "12900", "volume": "25240" }, { "symbol": "SSI", "value": "12900", "tradingdate": "04/05/2020", "time": "09:16:55", "open": "12900", "high": "12900", "low": "12900", "close": "12900", "volume": "6650" } ], "message": "SUCCESS", "status": "SUCCESS", "totalrecord": 200, "intradayohlcresponse": { } }

## 4.8 Daily Index (GetDailyIndex)

- Url: https://fc-data.ssi.com.vn/api/v2/Market/DailyIndex
- Method: GET

### GetDailyIndex Request

**No Name of Type Mandatory Description Valid value Element**

1. Indexcode String Y Index ID Valid values can be queried by api getIndexList

| 2. |  |  | Number | N | Value of the index | If not specify get today |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  | date |

1. ToDate Date N If not specify, get today date
2. PageIndex Number N From 1 to 10 Default 1
3. PageSize Number N 10; 20; 50; 100; 500; 1000 Default 10
4. OrderBy String N Default Tradingdate

<!-- PAGE 24 -->

1. Order String N Default desc

### GetDailyIndex Response

**No Name of Element Type Description Valid value**

1. Indexcode String Index ID
2. IndexValue Number Value of the index
3. Trading Date Date Trading Date
4. Time Timestamp Time
5. Change Number Change of index
6. RatioChange Number Ratio of Change

| 7. | TotalTrade | Number | Tổng số lệnh khớp (cả thông thường và thỏa thuận) |  |
| --- | --- | --- | --- | --- |
| 8. | Totalmatchvol | Number | Tổng matched quantity of normal orders |  |
| 9. | Totalmatchval | Number | Tổng matched value of normal orders |  |

1. TypeIndex Type of index
2. IndexName String Name of Index

| 12. | Advances | Number | Total number of symbols having price increase |  |
| --- | --- | --- | --- | --- |
| 13. | Nochanges | Number | Total number of symbols having price unchange |  |
| 14. | Declines | Number | Total number of symbols having price decrease |  |
| 15. | Ceiling | Number | Total number of symbols having last price = ceiling price |  |
| 16. | Floor | Number | Total number of symbols having last price = floor price |  |
| 17. | Totaldealvol | Number | Total matched quantity of puthrough orders |  |
| 18. | Totaldealval | Number | Total matched value of putthrough orders |  |
| 19. | Totalvol | Number | Total matched quantity of both normal and putthrough |  |

1. Totalval Number Total matched value of both normal and putthrough
2. TradingSession String Trading session - ATO: ATO
3. LO: Continuous
4. ATC: ATC
5. PT: Putthrough
6. BREAK: lunch break
7. C: Market Close
8. H: Market halted
9. Market String Market - HOSE
10. HNX
11. UPCOM

<!-- PAGE 25 -->

- DER
- BOND
- Exchange String Exchange - HOSE
- HNX

### Example

**Request Response**

Indexid:HNXindex { Fromdate: 04/05/2020 "dataList": [ Todate: 04/05/2020 { "indexid": "HNXIndex", "indexvalue": "105.72", "tradingdate": "04/05/2020", "time": "", "change": "-0.0112", "ratiochange": "-1.05", "totaltrade": "0", "totalmatchvol": "40342400", "totalmatchval": "295811440000", "typeindex": "Main", "indexname": "HNXIndex", "advances": "59", "nochanges": "56", "declines": "92", "ceilings": "15", "floors": "18", "totaldealvol": "1236772", "totaldealval": "20217698200", "totalvol": "41579172", "totalval": "316029138200", "tradingsession": "C" } ], "message": "SUCCESS", "status": "SUCCESS", "totalrecord": 1, "dailyindexresponse": { } }

## 4.9 Daily Stock Price (GetStockPrice)

- Url: https://fc-data.ssi.com.vn/api/v2/Market/DailyStockPrice
- Method: GET

### GetStockPrice Request

**No Name of Type Required Description Valid value Element**

<!-- PAGE 26 -->

1. Symbol String N Symbol of the security Include Stock, CW and Derivatives
2. FromDate String Y DD/MM/YYYY
3. ToDate String Y DD/MM/YYYY Default 10

| 4. | PageIndex | String | N | Number of page | Start from 1, default 1 |
| --- | --- | --- | --- | --- | --- |
| 5. | PageSize | String | N | Size of a page | 10; 20; 50; 100 |

1. Market String N Identify what market for - HOSE this symbol - HNX
2. UPCOM
3. DER
4. BOND

### GetStockPrice Response

**No. Field Type Description Valid Value/Format**

1. Tradingdate string Trading date
2. Symbol string
3. Pricechange string Price change
4. Perpricechange string Per price change
5. Ceilingprice string Ceilingprice
6. Floorprice string Floorprice
7. Refprice string Reference Price
8. Openprice string Open price
9. Highestprice string Highest price
10. Lowestprice string Lowest price
11. Closeprice string Closeprice
12. Averageprice string averageprice
13. Closepriceadjusted string Closepriceadjusted
14. Totalmatchvol string Total match volume
15. Totalmatchval string Total match value
16. Totaldealval string Total deal value
17. Totaldealvol string Total deal volume
18. Foreignbuyvoltotal string Total foreign buy volume
19. Foreigncurrentroom
20. Foreignsellvoltotal string Total foreign sell volume
21. Foreignbuyvaltotal string Total foreign buy volume
22. Toreignsellvaltotal string Total foreign sell value
23. Totalbuytrade string Total buy trade
24. Totalbuytradevol string Total buy trade volume
25. Totalselltrade string Total sell trade
26. Totalselltradevol string Total sell trade volume

| 27. |  | string | Net volume after netoff from |  |
| --- | --- | --- | --- | --- |
|  | Netforeivol |  | foreign Sell volume to Foreign |  |
|  |  |  | Buy volume |  |

1. String Net value after netofffrom Sell Netforeignval value to Foreign Buy value
2. Totaltradedvol String Total traded vol includes: matched, put and odd

<!-- PAGE 27 -->

| 30. | Totaltradedvalue | String | Total traded value includes: |  |
| --- | --- | --- | --- | --- |
|  |  |  | matched, put and odd |  |

### Example

**Request Response**

Symbol: SSI Fromdate: 04/05/2020 "dataList": [ Todate: 04/05/2020 { "tradingdate": "04/05/2020", "pricechange": "-300", "perpricechange": "-2.30", "ceilingprice": "13900", "floorprice": "12100", "refprice": "13000", "openprice": "12900", "highestprice": "13000", "lowestprice": "12700", "closeprice": "12700", "averageprice": "12816", "closepriceadjusted": "12700", "totalmatchvol": "2180310", "totalmatchval": "27943000000", "totaldealval": "0", "totaldealvol": "0", "foreignbuyvoltotal": "25310", "foreigncurrentroom": "253974102", "foreignsellvoltotal": "1117250", "foreignbuyvaltotal": "322702500", "foreignsellvaltotal": "14244937500", "totalbuytrade": "0", "totalbuytradevol": "0", "totalselltrade": "0", "totalselltradevol": "0", "netbuysellvol": "-1091940", "netbuysellval": "-13922235000", "totaltradedvol": "2180310", "totaltradedvalue": "27943000000", "symbol": "SSI", "time": "" } ], "message": "SUCCESS", "status": "SUCCESS", "totalrecord": 1, "stockpriceresponse": { } }
