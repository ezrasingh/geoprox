package org.geoprox.groovy.client.model;

import groovy.transform.Canonical
import io.swagger.annotations.ApiModel;
import io.swagger.annotations.ApiModelProperty;

@Canonical
class Neighbor {
    /* Distance in kilometers */
    Double distance
    /* Object key */
    String key
}
