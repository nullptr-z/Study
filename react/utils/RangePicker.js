import React, { Component, Fragment } from 'react';
import { Row, Col, Radio, DatePicker } from 'antd';
import moment from 'moment'

const RadioButton = Radio.Button;

export default class RangePicker extends Component {
  componentDidMount(){
    // this.getBeforeDate(this.props.defualtDay)
    this.getBeforeDate(1)
  }

  state = {
    radioIndex: undefined,
    startTime: '',
    endTime: ''
  }

  change = (radioIndex, startTime, endTime) => {
    const { onChange } = this.props
    onChange && onChange(radioIndex, startTime, endTime)
  }

  getBeforeDate = (day) => {
    const days = Number.parseInt(day)
    const today = new Date()
    const preTime = new Date(today.getTime() - days * 24 * 60 * 60 * 1000)
    const startTime = moment(preTime.setHours(9, 0, 0, 0))
    const endTime = moment(today.setHours(23, 59, 59, 999))
    this.setState({ radioIndex: days, startTime, endTime })
    this.change(days, startTime.format('YYYY-MM-DD HH:mm:ss'), endTime.format('YYYY-MM-DD HH:mm:ss'))
  }

  render() {
    const { radioIndex, startTime, endTime } = this.state
    const radioOptions = [1, 3, 7, 15, 30].map((value, index) => (<RadioButton key={index} value={value} >{value}天</RadioButton>));
    return (
      <Fragment>
        <Row gutter={10} type="flex" onChange={({ target }) => console.log('value', target.value)}>
          <Col>
            <Radio.Group
              size="small"
              value={radioIndex}
              onChange={({ target }) => this.getBeforeDate(target.value)}
            >
              {radioOptions}
            </Radio.Group>
          </Col>
          <Col>
            <DatePicker.RangePicker
              style={{ width: 370 }}
              size="small"
              ranges={{ '今天': [moment(), moment()] }}
              value={[startTime, endTime]}
              showTime
              format="YYYY-MM-DD HH:mm:ss"
              onChange={(date, dateStrings) => {
                this.setState({ startTime: date[0], endTime: date[1] })
                this.change(radioIndex, dateStrings[0], dateStrings[1])
              }}
            />
          </Col>
        </Row>
      </Fragment>
    );
  }
}

