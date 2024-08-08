package org.geoprox.groovy.client.model;

import groovy.transform.Canonical
import io.swagger.annotations.ApiModel;
import io.swagger.annotations.ApiModelProperty;

@Canonical
class DecodeGeohashResponse {
    /* Latitude */
    Double lat
    /* Latitude error */
    Double latError
    /* Longitude */
    Double lng
    /* Longitude error */
    Double lngError
}
