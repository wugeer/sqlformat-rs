use sqlformat::{format, FormatOptions, Indent, QueryParams};

fn main() {
    // let sql =
    //     "use mydb; with cte_table as (select id, name from test.my_table) select * from cte_table;select 1 from sld.dwxs";

    let sql = r#"SELECT
  table_0.* EXCEPT (profit),
  details.* EXCEPT (item_id),
  table_0.profit
FROM
  table_0"#;
    // 设置格式选项
    let options = FormatOptions {
        indent: Indent::Spaces(4), // 使用4个空格缩进
        uppercase: true,           // 将关键字大写
        lines_between_queries: 1,  // 查询之间空一行
    };

    // 格式化SQL
    let formatted_sql = format(sql, &QueryParams::None, options);

    println!("{}", formatted_sql);
}

/*
 -- 创建一个外部表，存储销售数据
CREATE EXTERNAL TABLE IF NOT EXISTS sales_data (
    -- 唯一标识订单ID
    order_id BIGINT COMMENT 'Unique identifier for the order',

    -- 客户ID
    customer_id BIGINT COMMENT 'Unique identifier for the customer',
)
COMMENT 'Sales data table for storing transaction records';

-- 按销售日期和城市进行分区
PARTITIONED BY (
    sale_year STRING COMMENT 'Year of the sale',
    sale_month STRING COMMENT 'Month of the sale'
)

-- 设置数据存储位置
LOCATION '/user/hive/warehouse/sales_data'

-- 使用 ORC 存储格式
STORED AS ORC

-- 设置表的行格式
ROW FORMAT DELIMITED
FIELDS TERMINATED BY ','
LINES TERMINATED BY '\n'

-- 设置表属性
TBLPROPERTIES (
    'orc.compress' = 'SNAPPY',          -- 使用SNAPPY压缩
    'transactional' = 'true',           -- 启用事务支持
    'orc.create.index' = 'true',        -- 创建索引
    'skip.header.line.count' = '1',     -- 跳过CSV文件的第一行
    'external.table.purge' = 'true'     -- 在删除表时自动清理数据
);

-- 自动加载数据到 Hive 分区中
ALTER TABLE sales_data
ADD PARTITION (sale_year = '2024', sale_month = '08')
LOCATION '/user/hive/warehouse/sales_data/2024/08';
*/
