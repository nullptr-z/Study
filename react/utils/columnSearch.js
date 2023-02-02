import { Button, Col, Icon, Input, Row } from "antd"

export function columnSearch(dataKey, searchCB, clearCB) {
  return {
    filterDropdown: ({ setSelectedKeys, selectedKeys, confirm, clearFilters }) => (
      <Row type="flex" gutter={2} style={{ padding: 10, borderRadius: 5 }}>
        <Col>
          <Input.Search
            enterButton                           // 回车触发搜索
            value={selectedKeys[0]}
            onChange={e => setSelectedKeys(e.target.value ? [e.target.value] : [])}
            placeholder="enter键确认搜索"
            onSearch={_ => {
              confirm()
              searchCB && searchCB(selectedKeys[0], dataKey)
            }}
          />
        </Col>
        <Col>
          <Button
            onClick={() => {
              clearFilters()
              clearCB && clearCB(selectedKeys[0], dataKey)
            }}
            icon="delete"
          />
        </Col>
      </Row>
    ),
    onFilter: (value, record) =>
      record[dataKey]
        .toString()
        .toLowerCase()
        .includes(value.toLowerCase()),
    filterIcon: _ => (
      <Icon type="search" />
    )
  }
}

export function mergeCells(value, index, dataList, predicate = () => true) {
  const obj = {
    children: value,
    props: {},
  };
  let i = index - 1;
  if (dataList[i] && predicate(dataList[i])) {
    obj.props.rowSpan = 0;
  } else {
    i = index + 1;
    while (dataList[i] && predicate(dataList[i]))
      i++;
    obj.props.rowSpan = i - index;
  }
  return obj;
}

export function groupBys(arr, filed) {
  const temp = {}

  arr.map((item) => {
    if (temp[item[filed]]) {
      temp[item[filed]].push(item);
    } else {
      temp[item[filed]] = []
      temp[item[filed]].push(item);
    }
  })
  return temp
}