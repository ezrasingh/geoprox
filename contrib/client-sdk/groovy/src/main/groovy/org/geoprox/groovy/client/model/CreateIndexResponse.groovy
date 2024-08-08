package org.geoprox.groovy.client.model;

import groovy.transform.Canonical
import io.swagger.annotations.ApiModel;
import io.swagger.annotations.ApiModelProperty;

@Canonical
class CreateIndexResponse {
    /* If true index was created */
    Boolean created
    /* If true index alredy exist */
    Boolean existed
}
