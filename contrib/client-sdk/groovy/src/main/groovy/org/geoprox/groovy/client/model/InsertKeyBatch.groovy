package org.geoprox.groovy.client.model;

import groovy.transform.Canonical
import io.swagger.annotations.ApiModel;
import io.swagger.annotations.ApiModelProperty;
import java.util.ArrayList;
import java.util.Arrays;
import org.geoprox.groovy.client.model.InsertKey;
import org.openapitools.jackson.nullable.JsonNullable;

@Canonical
class InsertKeyBatch {
    /* Object key */
    List<InsertKey> keys = new ArrayList<>()
    
    Boolean preserveOrder
    /* The time-to-live (TTL) for these keys, in seconds */
    Long ttl
}
