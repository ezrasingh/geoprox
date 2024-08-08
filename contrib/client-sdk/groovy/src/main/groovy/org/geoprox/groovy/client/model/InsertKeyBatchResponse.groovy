package org.geoprox.groovy.client.model;

import groovy.transform.Canonical
import io.swagger.annotations.ApiModel;
import io.swagger.annotations.ApiModelProperty;
import java.util.HashMap;

@Canonical
class InsertKeyBatchResponse {
    /* Contains information about which keys failed to be inserted and the associated error details. */
    Map<String, String> errors = new HashMap<>()
    /* Object keys that have been inserted in the index and their geohash. */
    Map<String, String> results = new HashMap<>()
}
