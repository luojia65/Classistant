import React from "react";

import {makeStyles} from "@material-ui/core/styles";
import {Avatar, Container, Divider, Grid, Typography} from "@material-ui/core";
import {AvatarGroup} from '@material-ui/lab';


const useStyles = makeStyles(theme => ({
        largeAva: {
            width: theme.spacing(10),
            height: theme.spacing(10),
        },
        memberAva: {
            width: theme.spacing(6),
            height: theme.spacing(6),
        },
        section1: {

        },
        section2: {
            margin: theme.spacing(2, 2),
        },
        divider: {
            marginTop: theme.spacing(1)
        }
    }))
;

let members:string[] = ["Alex A","Bob B","Cindy C","Dick D","Eric E"];

export default function GroupInfo(data:Object) {
    const classes = useStyles();
    return (
        <Container>
            <div>
                <Grid container alignItems="center">
                    <Grid item xs={3}>
                        <Avatar className={classes.largeAva}>G</Avatar>
                    </Grid>
                    <Grid item container xs={9}>
                        <Grid item xs={12} className={classes.section1}>
                            <Typography gutterBottom variant="h6">
                                GroupName
                            </Typography>
                        </Grid>
                        <Grid item xs={12} className={classes.section1}>
                            <Typography gutterBottom variant="body2">
                                Here is some description.
                            </Typography>
                            <Divider className={classes.divider} variant="middle"/>
                        </Grid>
                        <Grid item xs={12} className={classes.section2}>
                            <AvatarGroup spacing="small">
                                {members.map((i)=><Avatar className={classes.memberAva} alt={i}/>)}
                            </AvatarGroup>
                        </Grid>
                    </Grid>
                </Grid>
            </div>
        </Container>
    );
}