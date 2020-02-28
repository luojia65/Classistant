import React from "react";

import {makeStyles} from "@material-ui/core/styles";
import {Avatar, Container, Grid, Typography} from "@material-ui/core";
import {AvatarGroup} from '@material-ui/lab';


const useStyles = makeStyles(theme => ({
        largeAva: {
            width: theme.spacing(8),
            height: theme.spacing(8),
        },
        memberAva: {
            width: theme.spacing(6),
            height: theme.spacing(6),
        }
    }))
;
export default function GroupInfo() {
    const classes = useStyles();
    return (
        <Container>
            <div>
                <Grid container alignItems="center">
                    <Grid item xs={3}>
                        <Avatar className={classes.largeAva}>G</Avatar>
                    </Grid>
                    <Grid item container xs={9}>
                        <Grid item xs={12}>
                            <Typography gutterBottom variant="h6">
                                GroupName
                            </Typography>
                        </Grid>
                        <Grid item xs={12}>
                            <Typography gutterBottom variant="body2">
                                Here is some description.
                            </Typography>
                        </Grid>
                        <Grid item xs={12}>
                            <AvatarGroup spacing="small">
                                <Avatar className={classes.memberAva} alt="Alex A"/>
                                <Avatar className={classes.memberAva} alt="Bob B"/>
                                <Avatar className={classes.memberAva} alt="Cindy C"/>
                                <Avatar className={classes.memberAva} alt="Dick D"/>
                                <Avatar className={classes.memberAva} alt="Eric E"/>
                                <Avatar className={classes.memberAva} alt="Frank F"/>
                                <Avatar className={classes.memberAva} alt="Grey G"/>
                                <Avatar className={classes.memberAva}> +30</Avatar>
                            </AvatarGroup>
                        </Grid>
                    </Grid>


                </Grid>
            </div>
        </Container>
    );
}