package org.geoprox.groovy.client.model;

import groovy.transform.Canonical
import io.swagger.annotations.ApiModel;
import io.swagger.annotations.ApiModelProperty;

@Canonical
class InsertKeyResponse {
    /* Geohash encoded latitude/longitude */
    String geohash
    /* Object key */
    String key
}
