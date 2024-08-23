package org.geoprox.groovy.client.model;

import groovy.transform.Canonical
import io.swagger.annotations.ApiModel;
import io.swagger.annotations.ApiModelProperty;
import org.openapitools.jackson.nullable.JsonNullable;

@Canonical
class InsertKey {
    /* Object key */
    String key
    /* Latitude */
    Double lat
    /* Longitude */
    Double lng
    /* The time-to-live (TTL) for this key, in seconds */
    Long ttl
}
