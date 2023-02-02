import React, { Component } from 'react';
import { connect } from 'react-redux';
import { Graph, Shape } from '@antv/x6';
import { Button } from 'antd';
import { DataUri } from '@antv/x6'
import '@antv/x6-react-shape'
import { CardUnitCmp, UnitCmp } from './TableComponent/UnitCmp';

const bodyWidth = document.body.clientWidth

const data = {// 节点
    nodes: [
        {
            id: 'node1', // String，可选，节点的唯一标识
            x: 40,       // Number，必选，节点位置的 x 值
            y: 40,       // Number，必选，节点位置的 y 值
            width: 80,   // Number，可选，节点大小的 width 值
            height: 40,  // Number，可选，节点大小的 height 值
            label: '初始化', // String，节点标签
            shape: 'rect',   // 指定内置的形状
            attrs: {
                body: {
                    fill: '#2ECC71',
                    stroke: '#000',
                    strokeDasharray: '10,2',
                    rx: 16,
                    ry: 16,
                },
                label: {
                    text: '初始化',
                    fill: '#333',
                    fontSize: 13,
                }

            }
        },
        {
            id: 'node2', // String，节点的唯一标识
            x: 160,      // Number，必选，节点位置的 x 值
            y: 180,      // Number，必选，节点位置的 y 值
            width: 80,   // Number，可选，节点大小的 width 值
            height: 40,  // Number，可选，节点大小的 height 值
            label: 'world', // String，节点标签
            zIndex: 0,
            shape: 'ellipse'  // 指定内置的形状
        },
    ],
    edges: [  //  边/线
        {
            source: 'node1', // String，必须，起始节点 id
            target: 'node2', // String，必须，目标节点 id
            // shape: 'double-edge', // 样式
            attrs: {
                line: {
                    stroke: 'orange',
                },
            },
        },
    ],
};

class MainCharts extends Component {

    state = {
        graph: null,
        nodeNum: 1000,
        targets: [],
        x: 180,
        y: 180,
    }

    componentDidMount() {
        Graph.registerReactComponent('UnitCmp',
            <UnitCmp
                title="已到件"
                unit={['票数/件数', '重量', '体积']}
            />
        )
        this.init()
    }

    componentWillUnmount() {
        Graph.unregisterReactComponent('UnitCmp')
    }

    init() {
        const graph = new Graph({
            // panning: true, // 启动画布滚动 拖拽
            panning: {
                enabled: true,
                // modifiers: 'shift',  // 指定按键按下时启动拖拽
            },
            container: document.getElementById('container'),
            width: bodyWidth,
            height: this.props.tableOffsetHeight - 184,
            background: {
                color: '#7f7', // 设置画布背景颜色
            },
            grid: {
                size: 2,      // 网格大小 px
                visible: true, // 渲染网格背景
            }
        });

        this.setState({ graph })

        graph.fromJSON(data) // 渲染数据

        graph.centerContent()  // 内容居中

    }

    addNodes = () => {
        let { graph, nodeNum, targets, x, y } = this.state
        x += 150
        const source = new Shape.Rect({
            id: `node${nodeNum}`,
            x: x,
            y: 40,
            width: 100,
            height: 40,
            label: 'rect',
        })
        graph.addNode(source)
        const target = graph.addNode({
            x: x,
            y: y,
            width: 100,
            height: 40,
            shape: 'react-shape',
            component: 'UnitCmp',
        })

        const edge = new Shape.Edge({
            id: `edge${nodeNum}`,
            source: source,
            target: target,
            zIndex: 1,
        })
        graph.addEdge(edge)
        targets.push(target)
        this.setState({ nodeNum: nodeNum + 1, targets, x, y })
    }

    updateData() {
        this.state.targets.forEach(target => {
            target.prop('attrs/value', this.props.value)
        })
    }

    render() {
        const { graph } = this.state
        return (
            <div>
                <Button onClick={this.init.bind(this)}>初始化</Button>
                <Button onClick={() => graph.dispose()}>删除</Button>
                <Button onClick={this.addNodes}>添加节点</Button>
                {/* // 居中显示 */}
                <Button onClick={() => graph.centerContent()}>居中</Button>
                {/* // graph.zoom() // 获取缩放级别 */}
                {/* // graph.zoom(0.2) // 在原来缩放级别上增加 0.2 */}
                <Button onClick={() => graph.zoom(0.2)}>放大</Button>
                {/* // graph.zoom(-0.2) // 在原来缩放级别上减少 0.2 */}
                <Button onClick={() => graph.zoom(-0.2)}>缩小</Button>
                {/* // graph.translate(+2, -4) // 平移画布 */}
                <Button onClick={() => graph.translate(+2, -4)}>移动</Button>
                {/* // 方法进行画布的销毁以及资源的回收 */}
                <Button onClick={this.exportImg()}>导出为图片</Button>
                <Button onClick={this.updateData.bind(this)} type="primary">更新数据</Button>
                <div id="container">
                </div>
            </div>
        )
    }


    exportImg() {
        return () => {
            graph.toPNG(
                (dataUri) => {
                    DataUri.downloadDataUri(dataUri, 'chart.png');
                },
                {
                    width: 1000,
                    height: 1000,
                    backgroundColor: '#7f7'
                });
        };
    }

}

export default connect(({ centermassagemap }) => {
    return {
        value: centermassagemap.value
    }
})(MainCharts)
