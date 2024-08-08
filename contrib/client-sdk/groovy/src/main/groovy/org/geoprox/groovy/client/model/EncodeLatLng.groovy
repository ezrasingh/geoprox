package org.geoprox.groovy.client.model;

import groovy.transform.Canonical
import io.swagger.annotations.ApiModel;
import io.swagger.annotations.ApiModelProperty;

@Canonical
class EncodeLatLng {
    /* Determines geohash length */
    Integer depth
    /* Latitude */
    Double lat
    /* Longitude */
    Double lng
}
