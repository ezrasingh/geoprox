<?php
/**
 * GeohashNeighborsResponse
 *
 * PHP version 7.4
 *
 * @category Class
 * @package  OpenAPI\Client
 * @author   OpenAPI Generator team
 * @link     https://openapi-generator.tech
 */

/**
 * geoprox-server
 *
 * Geoprox server implementation providing a HTTP API for geospatial queries and position tracking
 *
 * The version of the OpenAPI document: 0.4.0
 * Contact: singhezra@gmail.com
 * Generated by: https://openapi-generator.tech
 * Generator version: 7.7.0
 */

/**
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

namespace OpenAPI\Client\Model;

use \ArrayAccess;
use \OpenAPI\Client\ObjectSerializer;

/**
 * GeohashNeighborsResponse Class Doc Comment
 *
 * @category Class
 * @description Neighboring geohash regions
 * @package  OpenAPI\Client
 * @author   OpenAPI Generator team
 * @link     https://openapi-generator.tech
 * @implements \ArrayAccess<string, mixed>
 */
class GeohashNeighborsResponse implements ModelInterface, ArrayAccess, \JsonSerializable
{
    public const DISCRIMINATOR = null;

    /**
      * The original name of the model.
      *
      * @var string
      */
    protected static $openAPIModelName = 'GeohashNeighborsResponse';

    /**
      * Array of property to type mappings. Used for (de)serialization
      *
      * @var string[]
      */
    protected static $openAPITypes = [
        'e' => 'string',
        'n' => 'string',
        'ne' => 'string',
        'nw' => 'string',
        's' => 'string',
        'se' => 'string',
        'sw' => 'string',
        'w' => 'string'
    ];

    /**
      * Array of property to format mappings. Used for (de)serialization
      *
      * @var string[]
      * @phpstan-var array<string, string|null>
      * @psalm-var array<string, string|null>
      */
    protected static $openAPIFormats = [
        'e' => null,
        'n' => null,
        'ne' => null,
        'nw' => null,
        's' => null,
        'se' => null,
        'sw' => null,
        'w' => null
    ];

    /**
      * Array of nullable properties. Used for (de)serialization
      *
      * @var boolean[]
      */
    protected static array $openAPINullables = [
        'e' => false,
        'n' => false,
        'ne' => false,
        'nw' => false,
        's' => false,
        'se' => false,
        'sw' => false,
        'w' => false
    ];

    /**
      * If a nullable field gets set to null, insert it here
      *
      * @var boolean[]
      */
    protected array $openAPINullablesSetToNull = [];

    /**
     * Array of property to type mappings. Used for (de)serialization
     *
     * @return array
     */
    public static function openAPITypes()
    {
        return self::$openAPITypes;
    }

    /**
     * Array of property to format mappings. Used for (de)serialization
     *
     * @return array
     */
    public static function openAPIFormats()
    {
        return self::$openAPIFormats;
    }

    /**
     * Array of nullable properties
     *
     * @return array
     */
    protected static function openAPINullables(): array
    {
        return self::$openAPINullables;
    }

    /**
     * Array of nullable field names deliberately set to null
     *
     * @return boolean[]
     */
    private function getOpenAPINullablesSetToNull(): array
    {
        return $this->openAPINullablesSetToNull;
    }

    /**
     * Setter - Array of nullable field names deliberately set to null
     *
     * @param boolean[] $openAPINullablesSetToNull
     */
    private function setOpenAPINullablesSetToNull(array $openAPINullablesSetToNull): void
    {
        $this->openAPINullablesSetToNull = $openAPINullablesSetToNull;
    }

    /**
     * Checks if a property is nullable
     *
     * @param string $property
     * @return bool
     */
    public static function isNullable(string $property): bool
    {
        return self::openAPINullables()[$property] ?? false;
    }

    /**
     * Checks if a nullable property is set to null.
     *
     * @param string $property
     * @return bool
     */
    public function isNullableSetToNull(string $property): bool
    {
        return in_array($property, $this->getOpenAPINullablesSetToNull(), true);
    }

    /**
     * Array of attributes where the key is the local name,
     * and the value is the original name
     *
     * @var string[]
     */
    protected static $attributeMap = [
        'e' => 'e',
        'n' => 'n',
        'ne' => 'ne',
        'nw' => 'nw',
        's' => 's',
        'se' => 'se',
        'sw' => 'sw',
        'w' => 'w'
    ];

    /**
     * Array of attributes to setter functions (for deserialization of responses)
     *
     * @var string[]
     */
    protected static $setters = [
        'e' => 'setE',
        'n' => 'setN',
        'ne' => 'setNe',
        'nw' => 'setNw',
        's' => 'setS',
        'se' => 'setSe',
        'sw' => 'setSw',
        'w' => 'setW'
    ];

    /**
     * Array of attributes to getter functions (for serialization of requests)
     *
     * @var string[]
     */
    protected static $getters = [
        'e' => 'getE',
        'n' => 'getN',
        'ne' => 'getNe',
        'nw' => 'getNw',
        's' => 'getS',
        'se' => 'getSe',
        'sw' => 'getSw',
        'w' => 'getW'
    ];

    /**
     * Array of attributes where the key is the local name,
     * and the value is the original name
     *
     * @return array
     */
    public static function attributeMap()
    {
        return self::$attributeMap;
    }

    /**
     * Array of attributes to setter functions (for deserialization of responses)
     *
     * @return array
     */
    public static function setters()
    {
        return self::$setters;
    }

    /**
     * Array of attributes to getter functions (for serialization of requests)
     *
     * @return array
     */
    public static function getters()
    {
        return self::$getters;
    }

    /**
     * The original name of the model.
     *
     * @return string
     */
    public function getModelName()
    {
        return self::$openAPIModelName;
    }


    /**
     * Associative array for storing property values
     *
     * @var mixed[]
     */
    protected $container = [];

    /**
     * Constructor
     *
     * @param mixed[] $data Associated array of property values
     *                      initializing the model
     */
    public function __construct(array $data = null)
    {
        $this->setIfExists('e', $data ?? [], null);
        $this->setIfExists('n', $data ?? [], null);
        $this->setIfExists('ne', $data ?? [], null);
        $this->setIfExists('nw', $data ?? [], null);
        $this->setIfExists('s', $data ?? [], null);
        $this->setIfExists('se', $data ?? [], null);
        $this->setIfExists('sw', $data ?? [], null);
        $this->setIfExists('w', $data ?? [], null);
    }

    /**
    * Sets $this->container[$variableName] to the given data or to the given default Value; if $variableName
    * is nullable and its value is set to null in the $fields array, then mark it as "set to null" in the
    * $this->openAPINullablesSetToNull array
    *
    * @param string $variableName
    * @param array  $fields
    * @param mixed  $defaultValue
    */
    private function setIfExists(string $variableName, array $fields, $defaultValue): void
    {
        if (self::isNullable($variableName) && array_key_exists($variableName, $fields) && is_null($fields[$variableName])) {
            $this->openAPINullablesSetToNull[] = $variableName;
        }

        $this->container[$variableName] = $fields[$variableName] ?? $defaultValue;
    }

    /**
     * Show all the invalid properties with reasons.
     *
     * @return array invalid properties with reasons
     */
    public function listInvalidProperties()
    {
        $invalidProperties = [];

        if ($this->container['e'] === null) {
            $invalidProperties[] = "'e' can't be null";
        }
        if ($this->container['n'] === null) {
            $invalidProperties[] = "'n' can't be null";
        }
        if ($this->container['ne'] === null) {
            $invalidProperties[] = "'ne' can't be null";
        }
        if ($this->container['nw'] === null) {
            $invalidProperties[] = "'nw' can't be null";
        }
        if ($this->container['s'] === null) {
            $invalidProperties[] = "'s' can't be null";
        }
        if ($this->container['se'] === null) {
            $invalidProperties[] = "'se' can't be null";
        }
        if ($this->container['sw'] === null) {
            $invalidProperties[] = "'sw' can't be null";
        }
        if ($this->container['w'] === null) {
            $invalidProperties[] = "'w' can't be null";
        }
        return $invalidProperties;
    }

    /**
     * Validate all the properties in the model
     * return true if all passed
     *
     * @return bool True if all properties are valid
     */
    public function valid()
    {
        return count($this->listInvalidProperties()) === 0;
    }


    /**
     * Gets e
     *
     * @return string
     */
    public function getE()
    {
        return $this->container['e'];
    }

    /**
     * Sets e
     *
     * @param string $e East
     *
     * @return self
     */
    public function setE($e)
    {
        if (is_null($e)) {
            throw new \InvalidArgumentException('non-nullable e cannot be null');
        }
        $this->container['e'] = $e;

        return $this;
    }

    /**
     * Gets n
     *
     * @return string
     */
    public function getN()
    {
        return $this->container['n'];
    }

    /**
     * Sets n
     *
     * @param string $n North
     *
     * @return self
     */
    public function setN($n)
    {
        if (is_null($n)) {
            throw new \InvalidArgumentException('non-nullable n cannot be null');
        }
        $this->container['n'] = $n;

        return $this;
    }

    /**
     * Gets ne
     *
     * @return string
     */
    public function getNe()
    {
        return $this->container['ne'];
    }

    /**
     * Sets ne
     *
     * @param string $ne North East
     *
     * @return self
     */
    public function setNe($ne)
    {
        if (is_null($ne)) {
            throw new \InvalidArgumentException('non-nullable ne cannot be null');
        }
        $this->container['ne'] = $ne;

        return $this;
    }

    /**
     * Gets nw
     *
     * @return string
     */
    public function getNw()
    {
        return $this->container['nw'];
    }

    /**
     * Sets nw
     *
     * @param string $nw North West
     *
     * @return self
     */
    public function setNw($nw)
    {
        if (is_null($nw)) {
            throw new \InvalidArgumentException('non-nullable nw cannot be null');
        }
        $this->container['nw'] = $nw;

        return $this;
    }

    /**
     * Gets s
     *
     * @return string
     */
    public function getS()
    {
        return $this->container['s'];
    }

    /**
     * Sets s
     *
     * @param string $s South
     *
     * @return self
     */
    public function setS($s)
    {
        if (is_null($s)) {
            throw new \InvalidArgumentException('non-nullable s cannot be null');
        }
        $this->container['s'] = $s;

        return $this;
    }

    /**
     * Gets se
     *
     * @return string
     */
    public function getSe()
    {
        return $this->container['se'];
    }

    /**
     * Sets se
     *
     * @param string $se South East
     *
     * @return self
     */
    public function setSe($se)
    {
        if (is_null($se)) {
            throw new \InvalidArgumentException('non-nullable se cannot be null');
        }
        $this->container['se'] = $se;

        return $this;
    }

    /**
     * Gets sw
     *
     * @return string
     */
    public function getSw()
    {
        return $this->container['sw'];
    }

    /**
     * Sets sw
     *
     * @param string $sw South West
     *
     * @return self
     */
    public function setSw($sw)
    {
        if (is_null($sw)) {
            throw new \InvalidArgumentException('non-nullable sw cannot be null');
        }
        $this->container['sw'] = $sw;

        return $this;
    }

    /**
     * Gets w
     *
     * @return string
     */
    public function getW()
    {
        return $this->container['w'];
    }

    /**
     * Sets w
     *
     * @param string $w West
     *
     * @return self
     */
    public function setW($w)
    {
        if (is_null($w)) {
            throw new \InvalidArgumentException('non-nullable w cannot be null');
        }
        $this->container['w'] = $w;

        return $this;
    }
    /**
     * Returns true if offset exists. False otherwise.
     *
     * @param integer $offset Offset
     *
     * @return boolean
     */
    public function offsetExists($offset): bool
    {
        return isset($this->container[$offset]);
    }

    /**
     * Gets offset.
     *
     * @param integer $offset Offset
     *
     * @return mixed|null
     */
    #[\ReturnTypeWillChange]
    public function offsetGet($offset)
    {
        return $this->container[$offset] ?? null;
    }

    /**
     * Sets value based on offset.
     *
     * @param int|null $offset Offset
     * @param mixed    $value  Value to be set
     *
     * @return void
     */
    public function offsetSet($offset, $value): void
    {
        if (is_null($offset)) {
            $this->container[] = $value;
        } else {
            $this->container[$offset] = $value;
        }
    }

    /**
     * Unsets offset.
     *
     * @param integer $offset Offset
     *
     * @return void
     */
    public function offsetUnset($offset): void
    {
        unset($this->container[$offset]);
    }

    /**
     * Serializes the object to a value that can be serialized natively by json_encode().
     * @link https://www.php.net/manual/en/jsonserializable.jsonserialize.php
     *
     * @return mixed Returns data which can be serialized by json_encode(), which is a value
     * of any type other than a resource.
     */
    #[\ReturnTypeWillChange]
    public function jsonSerialize()
    {
       return ObjectSerializer::sanitizeForSerialization($this);
    }

    /**
     * Gets the string presentation of the object
     *
     * @return string
     */
    public function __toString()
    {
        return json_encode(
            ObjectSerializer::sanitizeForSerialization($this),
            JSON_PRETTY_PRINT
        );
    }

    /**
     * Gets a header-safe presentation of the object
     *
     * @return string
     */
    public function toHeaderValue()
    {
        return json_encode(ObjectSerializer::sanitizeForSerialization($this));
    }
}

