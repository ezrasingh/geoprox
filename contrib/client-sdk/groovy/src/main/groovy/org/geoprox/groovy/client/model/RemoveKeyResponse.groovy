package org.geoprox.groovy.client.model;

import groovy.transform.Canonical
import io.swagger.annotations.ApiModel;
import io.swagger.annotations.ApiModelProperty;

@Canonical
class RemoveKeyResponse {
    /* If true key was removed */
    Boolean deleted
    /* Object key */
    String key
}
