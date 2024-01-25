<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Cancelación de Envíos</name>
   <tag></tag>
   <elementGuidId>599ae1c7-ff1c-44bb-8ae2-ff7fde020e56</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n    \&quot;status\&quot;: \&quot;CANCEL\&quot;,\r\n    \&quot;tracking_number\&quot;: \&quot;2200000123\&quot;\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>8.6.8</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${base_url}/shipments/238461836588366/authorization</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.base_url</defaultValue>
      <description></description>
      <id>c4c7980c-766a-45cb-b968-f71d6638f963</id>
      <masked>false</masked>
      <name>base_url</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
