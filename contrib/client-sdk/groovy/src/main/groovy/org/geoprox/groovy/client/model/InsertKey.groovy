package org.geoprox.groovy.client.model;

import groovy.transform.Canonical
import io.swagger.annotations.ApiModel;
import io.swagger.annotations.ApiModelProperty;

@Canonical
class InsertKey {
    /* Object key */
    String key
    /* Latitude */
    Double lat
    /* Longitude */
    Double lng
}
