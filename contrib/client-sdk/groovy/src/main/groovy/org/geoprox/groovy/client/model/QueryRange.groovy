package org.geoprox.groovy.client.model;

import groovy.transform.Canonical
import io.swagger.annotations.ApiModel;
import io.swagger.annotations.ApiModelProperty;
import org.openapitools.jackson.nullable.JsonNullable;

@Canonical
class QueryRange {
    /* Maximum number of neighbors that can be returned (default 100) */
    Integer count
    /* Latitude */
    Double lat
    /* Longitude */
    Double lng
    /* Search radius in kilometers */
    Integer range
    /* If enabled neighbors will be sorted by distance, nearest to furthest (default false) */
    Boolean sorted
}
