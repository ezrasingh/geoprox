package org.geoprox.groovy.client.model;

import groovy.transform.Canonical
import io.swagger.annotations.ApiModel;
import io.swagger.annotations.ApiModelProperty;
import java.util.HashMap;
import org.geoprox.groovy.client.model.Neighbor;

@Canonical
class QueryRangeManyResponse {
    /* Contains information about any errors occured during batch search. */
    Map<String, String> errors = new HashMap<>()
    /* Object keys found within range */
    Map<String, List<Neighbor>> results = new HashMap<>()
}
