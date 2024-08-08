package org.geoprox.groovy.client.model;

import groovy.transform.Canonical
import io.swagger.annotations.ApiModel;
import io.swagger.annotations.ApiModelProperty;

@Canonical
class DropIndexResponse {
    /* If true index was deleted */
    Boolean deleted
}
