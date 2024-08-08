package org.geoprox.groovy.client.model;

import groovy.transform.Canonical
import io.swagger.annotations.ApiModel;
import io.swagger.annotations.ApiModelProperty;
import java.util.ArrayList;
import java.util.Arrays;
import org.openapitools.jackson.nullable.JsonNullable;

@Canonical
class QueryRangeMany {
    /* Maximum number of neighbors that can be returned (default 100) */
    Integer count
    /* List of indices to search */
    List<String> indices = new ArrayList<>()
    /* Latitude */
    Double lat
    /* Longitude */
    Double lng
    /* Search radius in kilometers */
    Integer range
    /* If enabled neighbors will be sorted by distance, nearest to furthest (default false) */
    Boolean sorted
}
