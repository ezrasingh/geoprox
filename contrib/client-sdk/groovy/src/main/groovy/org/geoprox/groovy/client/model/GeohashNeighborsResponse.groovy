package org.geoprox.groovy.client.model;

import groovy.transform.Canonical
import io.swagger.annotations.ApiModel;
import io.swagger.annotations.ApiModelProperty;

@Canonical
class GeohashNeighborsResponse {
    /* East */
    String e
    /* North */
    String n
    /* North East */
    String ne
    /* North West */
    String nw
    /* South */
    String s
    /* South East */
    String se
    /* South West */
    String sw
    /* West */
    String w
}
