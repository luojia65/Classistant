import React, {Component} from "react";
import {WithStyles, withStyles, Theme, Paper} from '@material-ui/core';
import {Avatar, Box, Tab, Tabs, Typography} from "@material-ui/core";



const styles = (theme: Theme) => ({
    largeAva: {
        width: theme.spacing(10),
        height: theme.spacing(10),
    }
});

interface GroupProps extends WithStyles<typeof styles> {
    id: number
}

type GroupState = {
    pageIndex: number,
    name: string,
    description: string
}

interface TabPanelProps {
    children?: React.ReactNode;
    dir?: string;
    index: any;
    value: any;
}


function Tasks(data: Object) {
    return(
        <Paper></Paper>
    );
}

function TabPanel(props: TabPanelProps) {
    const {children, value, index, ...other} = props;

    return (
        <Typography
            component="div"
            role="tabpanel"
            hidden={value !== index}
            id={`full-width-tabpanel-${index}`}
            aria-labelledby={`full-width-tab-${index}`}
            {...other}
        >
            {value === index && <Box p={3}>{children}</Box>}
        </Typography>
    );
}

function a11yProps(index: any) {
    return {
        id: `full-width-tab-${index}`,
        'aria-controls': `full-width-tabpanel-${index}`,
    };
}

class GroupPage extends Component<GroupProps, GroupState> {

    handleChange = (event: React.ChangeEvent<{}>, newValue: number) => {
        this.setState({pageIndex: newValue});
    };

    componentWillMount() {
        this.setState({pageIndex: 0});
        //TODO: 请求数据？
    }

    render() {
        const {classes} = this.props;
        return (
            <div>
                {/*<Avatar className={classes.largeAva}>G</Avatar>*/}
                <Typography variant="h6">Group Name</Typography>
                <Tabs
                    value={this.state.pageIndex}
                    onChange={this.handleChange}
                    indicatorColor="primary"
                    textColor="primary"
                    centered
                >
                    <Tab label="Tasks"/>
                    <Tab label="Members"/>
                    <Tab label="Item Three"/>
                </Tabs>
                <TabPanel value={this.state.pageIndex} index={0}>
                    Item One
                    <Tasks></Tasks>
                </TabPanel>
                <TabPanel value={this.state.pageIndex} index={1}>
                    Item Two
                </TabPanel>
                <TabPanel value={this.state.pageIndex} index={2}>
                    Item Three
                </TabPanel>
            </div>
        )
    }
}

export default withStyles(styles)(GroupPage);