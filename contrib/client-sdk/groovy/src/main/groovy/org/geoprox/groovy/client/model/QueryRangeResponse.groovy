package org.geoprox.groovy.client.model;

import groovy.transform.Canonical
import io.swagger.annotations.ApiModel;
import io.swagger.annotations.ApiModelProperty;
import java.util.ArrayList;
import java.util.Arrays;
import org.geoprox.groovy.client.model.Neighbor;

@Canonical
class QueryRangeResponse {
    /* Object keys found within range */
    List<Neighbor> found = new ArrayList<>()
}
